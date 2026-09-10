use anstyle::{Ansi256Color, AnsiColor, Style};
use cfgdrift::Difference;

const STYLE_CHANGED: Style = AnsiColor::Yellow.on_default();
const STYLE_ONLY_LEFT: Style = Ansi256Color(166).on_default();
const STYLE_ONLY_RIGHT: Style = AnsiColor::Green.on_default();
const STYLE_ERROR: Style = AnsiColor::Red.on_default();
const STYLE_CAUSE: Style = Style::new().dimmed();

pub fn print_diff(writer: &mut impl std::io::Write, diffs: &[Difference]) -> std::io::Result<()> {
    if diffs.is_empty() {
        writeln!(writer, "Конфигурации идентичны")?;
        return Ok(());
    }

    writeln!(writer, "Найдены различия:")?;
    writeln!(
        writer,
        "{STYLE_CHANGED}  ~  значение изменилось (было → стало){STYLE_CAUSE:#}"
    )?;
    writeln!(
        writer,
        "{STYLE_ONLY_LEFT}  -  ключ есть только в первом файле{STYLE_ONLY_LEFT:#}"
    )?;
    writeln!(
        writer,
        "{STYLE_ONLY_RIGHT}  +  ключ есть только во втором файле{STYLE_ONLY_RIGHT:#}"
    )?;
    writeln!(writer)?;

    let (mut changed, mut only_left, mut only_right) = (0, 0, 0);
    for diff in diffs {
        match diff {
            Difference::Mismatch {
                key,
                left_value,
                right_value,
            } => {
                writeln!(
                    writer,
                    "{STYLE_CHANGED}~ {key}: {left_value} → {right_value}{STYLE_CAUSE:#}"
                )?;
                changed += 1;
            }
            Difference::OnlyInLeft { key, value } => {
                writeln!(
                    writer,
                    "{STYLE_ONLY_LEFT}- {key}: {value}{STYLE_ONLY_LEFT:#}"
                )?;
                only_left += 1;
            }
            Difference::OnlyInRight { key, value } => {
                writeln!(
                    writer,
                    "{STYLE_ONLY_RIGHT}+ {key}: {value}{STYLE_ONLY_RIGHT:#}"
                )?;
                only_right += 1;
            }
        };
    }

    writeln!(
        writer,
        "\nитого: {} (изменено {changed}, только в первом {only_left}, только во втором {only_right})",
        diffs.len(),
    )?;

    Ok(())
}

pub fn print_error(
    writer: &mut impl std::io::Write,
    err: &dyn std::error::Error,
) -> std::io::Result<()> {
    writeln!(writer, "{STYLE_ERROR}ошибка:{STYLE_ERROR:#} {err}")?;

    let mut source = err.source();
    while let Some(cause) = source {
        writeln!(writer, "{STYLE_CAUSE}  причина: {cause}{STYLE_CAUSE:#}")?;
        source = cause.source();
    }

    Ok(())
}
