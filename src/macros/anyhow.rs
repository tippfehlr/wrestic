macro_rules! error {
    ($err:expr) => {
        anyhow::anyhow!(color_print::cformat!(
            r#"

            <k>###############################################</k>
            <r>{}</r>
            <g>{}</g> line <y>{}</y> column <y>{}</y>
            <k>###############################################</k>
            "#,
            $err,
            file!(),
            line!(),
            column!()
        ))
    };
}

macro_rules! uerror {
    ($err:expr) => {
        anyhow::anyhow!(color_print::cformat!("<r>{}</r>", $err))
    };
}
pub(crate) use error;
pub(crate) use uerror;