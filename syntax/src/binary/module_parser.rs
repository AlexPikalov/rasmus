use nom::{
    bytes::complete::{tag, take},
    IResult as NomResult,
};

use super::module::*;
use super::parse_trait::*;
use super::types::*;
pub struct ModuleParser;

macro_rules! get_section_content {
    ($bytes:expr) => {{
        let (bytes, (section_id_byte, section_content)) =
            Self::take_section($bytes).map_err(|_| SyntaxError::InvalidModuleSection)?;
        (
            bytes,
            SectionId::try_from(section_id_byte)?,
            section_content,
        )
    }};
}

impl ModuleParser {
    pub fn new() -> Self {
        ModuleParser {}
    }

    fn take_magic(bytes: &[Byte]) -> NomResult<&[Byte], ()> {
        tag(Module::MAGIC)(bytes).map(|(input, _)| (input, ()))
    }

    fn take_version(bytes: &[Byte]) -> NomResult<&[Byte], ()> {
        tag(Module::VERSION)(bytes).map(|(input, _)| (input, ()))
    }

    fn take_section(bytes: &[Byte]) -> NomResult<&[Byte], (u8, &[Byte])> {
        let (bytes, section_id_bytes) = take(1usize)(bytes)?;
        let (bytes, size) = U32Type::parse(bytes)?;
        let (bytes, section_content) = take(size.0)(bytes)?;

        Ok((bytes, (section_id_bytes[0], section_content)))
    }

    fn parse_custom_section(bytes: &[Byte]) -> ParseResult<CustomSection> {
        let (bytes, name) =
            NameType::parse(bytes).map_err(|_| SyntaxError::InvalidModuleSection)?;
        Ok(CustomSection {
            name: name.0,
            bytes: bytes.to_vec(),
        })
    }

    fn parse_types_section(bytes: &[Byte]) -> ParseResult<Vec<FuncType>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed =
            U32Type::parse(remaining_bytes).map_err(|_| SyntaxError::InvalidTypesModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut func_types: Vec<FuncType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let func_type_parsed = Self::parse_func_type(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidTypesModuleSection)?;
            remaining_bytes = func_type_parsed.0;
            func_types.push(func_type_parsed.1);
        }

        Ok(func_types)
    }

    fn parse_func_type(bytes: &[Byte]) -> NomResult<&[Byte], FuncType> {
        let (bytes, _) = tag(&[FuncType::ENCODE_BYTE])(bytes)?;
        let (bytes, parameters_vec_len) = U32Type::parse(bytes)?;

        let mut remaining_bytes = bytes;

        let mut parameters: Vec<ValType> = Vec::with_capacity(parameters_vec_len.0 as usize);
        for _ in 0..parameters_vec_len.0 {
            let parsed_val_type = ValType::parse(remaining_bytes)?;
            parameters.push(parsed_val_type.1);
            remaining_bytes = parsed_val_type.0;
        }

        let (bytes, results_vec_len) = U32Type::parse(remaining_bytes)?;
        remaining_bytes = bytes;
        let mut results: Vec<ValType> = Vec::with_capacity(results_vec_len.0 as usize);
        for _ in 0..results_vec_len.0 {
            let parsed_val_type = ValType::parse(remaining_bytes)?;
            results.push(parsed_val_type.1);
            remaining_bytes = parsed_val_type.0;
        }

        Ok((
            remaining_bytes,
            FuncType {
                parameters,
                results,
            },
        ))
    }

    fn parse_code_section(bytes: &[Byte]) -> ParseResult<Vec<CodeType>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed =
            U32Type::parse(remaining_bytes).map_err(|_| SyntaxError::InvalidCodeModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut code_types: Vec<CodeType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let code_type_parsed = Self::parse_code_type(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidCodeModuleSection)?;
            remaining_bytes = code_type_parsed.0;
            code_types.push(code_type_parsed.1);
        }

        Ok(code_types)
    }

    fn parse_code_type(bytes: &[Byte]) -> NomResult<&[Byte], CodeType> {
        let (bytes, code_len) = U32Type::parse(bytes)?;
        let (bytes, code_bytes) = take(code_len.0 as usize)(bytes)?;

        let (code_bytes, locals_len) = U32Type::parse(code_bytes)?;
        let mut remaining_bytes = code_bytes;
        let mut locals: Vec<LocalsType> = Vec::with_capacity(locals_len.0 as usize);

        for _ in 0..locals_len.0 {
            let locals_type_parsed = LocalsType::parse(remaining_bytes)?;
            remaining_bytes = locals_type_parsed.0;
            locals.push(locals_type_parsed.1);
        }

        let (_, expression) = ExpressionType::parse(code_bytes)?;

        Ok((
            bytes,
            CodeType {
                size: code_len,
                code: FuncCodeType { locals, expression },
            },
        ))
    }

    fn parse_funcs_section(bytes: &[Byte]) -> ParseResult<Vec<TypeIdx>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed =
            U32Type::parse(remaining_bytes).map_err(|_| SyntaxError::InvalidFuncsModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut func_types: Vec<TypeIdx> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let code_type_parsed = U32Type::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidFuncsModuleSection)?;
            remaining_bytes = code_type_parsed.0;
            func_types.push(TypeIdx(code_type_parsed.1));
        }

        Ok(func_types)
    }

    fn parse_import_section(bytes: &[Byte]) -> ParseResult<Vec<ImportType>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)
            .map_err(|_| SyntaxError::InvalidImportsModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut import_types: Vec<ImportType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let module_name_parsed = NameType::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidImportsModuleSection)?;
            let module_name = module_name_parsed.1;
            remaining_bytes = module_name_parsed.0;

            let func_name_parsed = NameType::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidImportsModuleSection)?;
            let func_name = func_name_parsed.1;
            remaining_bytes = func_name_parsed.0;

            let import_desc_parsed = Self::parse_import_desc_type(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidImportsModuleSection)?;
            let import_desc = import_desc_parsed.1;
            remaining_bytes = import_desc_parsed.0;

            import_types.push(ImportType {
                module: module_name,
                name: func_name,
                desc: import_desc,
            })
        }

        Ok(import_types)
    }

    fn parse_import_desc_type(bytes: &[Byte]) -> NomResult<&[Byte], ImportDescription> {
        let (bytes, encode_byte) =
            take(1usize)(bytes).map(|(b, encode_byte_slice)| (b, encode_byte_slice[0]))?;

        match encode_byte {
            ImportDescription::ENCODE_BYTE_FUNC => U32Type::parse(bytes)
                .map(|(b, u32_val)| (b, ImportDescription::Func(TypeIdx(u32_val)))),
            ImportDescription::ENCODE_BYTE_TABLE => {
                TableType::parse(bytes).map(|(b, val)| (b, ImportDescription::Table(val)))
            }
            ImportDescription::ENCODE_BYTE_MEM => {
                MemType::parse(bytes).map(|(b, val)| (b, ImportDescription::Mem(val)))
            }
            ImportDescription::ENCODE_BYTE_GLOBAL => {
                GlobalType::parse(bytes).map(|(b, val)| (b, ImportDescription::Global(val)))
            }
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Char,
            ))),
        }
    }

    fn parse_table_section(bytes: &[Byte]) -> ParseResult<Vec<TableType>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed =
            U32Type::parse(remaining_bytes).map_err(|_| SyntaxError::InvalidTablesModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut table_types: Vec<TableType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let table_type_parsed = TableType::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidTablesModuleSection)?;

            remaining_bytes = table_type_parsed.0;
            table_types.push(table_type_parsed.1);
        }

        Ok(table_types)
    }

    fn parse_memory_section(bytes: &[Byte]) -> ParseResult<Vec<MemType>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed =
            U32Type::parse(remaining_bytes).map_err(|_| SyntaxError::InvalidMemsModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut mem_types: Vec<MemType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let mem_type_parsed = MemType::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidMemsModuleSection)?;

            remaining_bytes = mem_type_parsed.0;
            mem_types.push(mem_type_parsed.1);
        }

        Ok(mem_types)
    }

    fn parse_globals_section(bytes: &[Byte]) -> ParseResult<Vec<GlobalType>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)
            .map_err(|_| SyntaxError::InvalidGlobalsModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut global_types: Vec<GlobalType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let global_type_parsed = GlobalType::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidGlobalsModuleSection)?;

            remaining_bytes = global_type_parsed.0;
            global_types.push(global_type_parsed.1);
        }

        Ok(global_types)
    }

    fn parse_exports_section(bytes: &[Byte]) -> ParseResult<Vec<ExportType>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)
            .map_err(|_| SyntaxError::InvalidImportsModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut export_types: Vec<ExportType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let func_name_parsed = NameType::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidImportsModuleSection)?;
            let func_name = func_name_parsed.1;
            remaining_bytes = func_name_parsed.0;

            let export_desc_parsed = Self::parse_export_desc_type(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidImportsModuleSection)?;
            let export_desc = export_desc_parsed.1;
            remaining_bytes = export_desc_parsed.0;

            export_types.push(ExportType {
                name: func_name,
                desc: export_desc,
            })
        }

        Ok(export_types)
    }

    fn parse_export_desc_type(bytes: &[Byte]) -> NomResult<&[Byte], ExportDescription> {
        let (bytes, encode_byte) =
            take(1usize)(bytes).map(|(b, encode_byte_slice)| (b, encode_byte_slice[0]))?;

        match encode_byte {
            ExportDescription::ENCODE_BYTE_FUNC => U32Type::parse(bytes)
                .map(|(b, u32_val)| (b, ExportDescription::Func(TypeIdx(u32_val)))),
            ExportDescription::ENCODE_BYTE_TABLE => U32Type::parse(bytes)
                .map(|(b, u32_val)| (b, ExportDescription::Table(TableIdx(u32_val)))),
            ExportDescription::ENCODE_BYTE_MEM => U32Type::parse(bytes)
                .map(|(b, u32_val)| (b, ExportDescription::Mem(MemIdx(u32_val)))),
            ExportDescription::ENCODE_BYTE_GLOBAL => U32Type::parse(bytes)
                .map(|(b, u32_val)| (b, ExportDescription::Global(GlobalIdx(u32_val)))),
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Char,
            ))),
        }
    }

    fn parse_start_section(bytes: &[Byte]) -> ParseResult<StartType> {
        StartType::parse(bytes)
            .map_err(|_| SyntaxError::InvalidStartModuleSection)
            .map(|v| v.1)
    }

    fn parse_elems_section(bytes: &[Byte]) -> ParseResult<Vec<ElementSegmentType>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)
            .map_err(|_| SyntaxError::InvalidImportsModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut elems_types: Vec<ElementSegmentType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let elem_segment_type_parsed = ElementSegmentType::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidElementSegmentModuleSection)?;

            remaining_bytes = elem_segment_type_parsed.0;
            elems_types.push(elem_segment_type_parsed.1);
        }

        Ok(elems_types)
    }
}

impl ParseBin<Module> for ModuleParser {
    fn parse(&mut self, bytes: &[Byte]) -> ParseResult<(Vec<Byte>, Module)> {
        let mut remainig_bytes = bytes;
        // FIXME: Is it possible to have few modules declared in the same file?
        remainig_bytes = Self::take_magic(remainig_bytes)
            .map_err(|_| SyntaxError::ModuleMagicNotFound)?
            .0;
        remainig_bytes = Self::take_version(remainig_bytes)
            .map_err(|_| SyntaxError::ModuleVersionNotFound)?
            .0;

        let mut module = Module::default();

        while !remainig_bytes.is_empty() {
            let (b, section_id, section_content) = get_section_content!(remainig_bytes);
            match section_id {
                SectionId::Custom => {
                    // custom sections are not a part of the Module structure, so ignore so far
                    println!("{:?}", Self::parse_custom_section(section_content))
                }
                SectionId::Type => module.types = Self::parse_types_section(section_content)?,
                SectionId::Code => module.code = Self::parse_code_section(section_content)?,
                SectionId::Function => module.funcs = Self::parse_funcs_section(section_content)?,
                SectionId::Import => module.imports = Self::parse_import_section(section_content)?,
                SectionId::Table => module.tables = Self::parse_table_section(bytes)?,
                SectionId::Memory => module.mems = Self::parse_memory_section(bytes)?,
                SectionId::Global => module.globals = Self::parse_globals_section(bytes)?,
                SectionId::Export => module.exports = Self::parse_exports_section(bytes)?,
                SectionId::Start => module.start = Some(Self::parse_start_section(bytes)?),
                SectionId::Element => module.elems = Self::parse_elems_section(bytes)?,
                _ => println!("unhandled Section ID {:?}", section_id),
            }
            println!("Section found: {:?}", section_id);
            println!("Section content: {:?}", section_content);
            // TODO: parse section content according to section id
            remainig_bytes = b;
        }

        Ok((remainig_bytes.to_vec(), module))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_module() {
        let wasm = std::fs::read(format!(
            "{}/wasm_files/empty_module.wasm",
            std::env::var("CARGO_MANIFEST_DIR").unwrap()
        ))
        .unwrap();
        let mut parser = ModuleParser::new();
        parser.parse(&wasm).unwrap();
    }

    #[test]
    fn test_module_with_function() {
        let wasm = std::fs::read(format!(
            "{}/wasm_files/with_function.wasm",
            std::env::var("CARGO_MANIFEST_DIR").unwrap()
        ))
        .unwrap();

        let mut parser = ModuleParser::new();
        let (_, module) = parser.parse(&wasm).unwrap();

        println!("TYPES: {:?}", module.types);
        println!("CODE: {:?}", module.code);
        println!("FUNCTIONS: {:?}", module.funcs);
        println!("TABLES: {:?}", module.tables);
        println!("MEMS: {:?}", module.mems);
        // println!("GLOBALS: {:?}", module.code);

        assert_eq!(
            module.types,
            vec![FuncType {
                parameters: vec![
                    ValType::NumType(NumType::I32),
                    ValType::NumType(NumType::I32)
                ],
                results: vec![ValType::NumType(NumType::I32)]
            }]
        );
    }
}
