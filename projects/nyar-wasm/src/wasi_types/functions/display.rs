use super::*;
use crate::helpers::ComponentDefine;

impl ComponentDefine for WasiFunction {
    fn wasi_define<W: Write>(&self, w: &mut WastEncoder<W>) -> std::fmt::Result {
        todo!()
    }

    fn alias_outer<W: Write>(&self, w: &mut WastEncoder<W>) -> std::fmt::Result {
        todo!()
    }

    fn alias_export<W: Write>(&self, w: &mut WastEncoder<W>, module: &WasiModule) -> std::fmt::Result {
        let id = self.symbol.wasi_id();
        match &self.body {
            WasiFunctionBody::External { wasi_name, .. } => write!(w, "(alias export ${module} \"{wasi_name}\" (func {id}))")?,
            WasiFunctionBody::Normal { .. } => {}
        }
        Ok(())
    }
}

impl Display for WasiFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.symbol)?;
        for (i, input) in self.inputs.iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?
            }
            match input.name.as_ref().eq("self") {
                true => f.write_str("self")?,
                false => write!(f, "{}: {:#}", input.name, input.r#type)?,
            }
        }
        f.write_char(')')
    }
}

impl LowerTypes for WasiFunction {
    fn canon_lower<W: Write>(&self, w: &mut WastEncoder<W>) -> std::fmt::Result {
        match &self.body {
            WasiFunctionBody::External { wasi_module, wasi_name } => {
                write!(w, "(core func {} (canon lower", self.symbol.wasi_id())?;
                w.indent();
                w.newline()?;
                write!(w, "(func ${} \"{}\")", wasi_module, wasi_name)?;
                if self.need_heap() {
                    w.newline()?;
                    write!(w, "(memory $memory \"memory\")")?;
                    write!(w, "(realloc (func $memory \"realloc\"))")?;
                }
                if let Some(encode) = self.need_encoding() {
                    w.newline()?;
                    write!(w, "string-encoding={encode}")?;
                }
                w.dedent(2);
            }
            WasiFunctionBody::Normal { .. } => {}
        }
        Ok(())
    }

    fn wasm_define<W: Write>(&self, w: &mut WastEncoder<W>) -> std::fmt::Result {
        match &self.body {
            WasiFunctionBody::External { wasi_module, wasi_name } => {
                write!(w, "(import \"{}\" \"{}\" (func {}", wasi_module, wasi_name, self.symbol.wasi_id())?;
            }
            WasiFunctionBody::Normal { .. } => {
                write!(w, "(func {}", self.symbol.wasi_id())?;
            }
        }
        w.indent();
        for input in &self.inputs {
            w.newline()?;
            input.lower_input(w)?;
        }
        for output in &self.output {
            w.newline()?;
            output.lower_input(w)?;
        }
        match &self.body {
            WasiFunctionBody::External { .. } => w.dedent(2),
            WasiFunctionBody::Normal { .. } => w.dedent(1),
        }
        Ok(())
    }
}
