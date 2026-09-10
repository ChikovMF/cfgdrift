use cfgdrift::Difference;

pub fn print_diff(writer: &mut impl std::io::Write, diffs: &[Difference]) -> std::io::Result<()> {
    if diffs.is_empty() {
        writeln!(writer, "Конфигурации идентичны")?;
        return Ok(());
    }

    writeln!(writer, "Найдены различия в конфигурациях (~ изменено, - только слева, + только справа):")?;
    let (mut changed, mut only_left, mut only_right) = (0, 0, 0);
    for diff in diffs {
        match diff {
            Difference::Mismatch {
                key,
                left_value,
                right_value,
            } => {
                writeln!(writer, "~ {key}: {left_value} → {right_value}")?;
                changed += 1;
            }
            Difference::OnlyInLeft { key, value } => {
                writeln!(writer, "- {key}: {value}\t(только в левом)")?;
                only_left += 1;
            }
            Difference::OnlyInRight { key, value } => {
                writeln!(writer, "+ {key}: {value}\t(только в правом)")?;
                only_right += 1;
            }
        };
    }

    writeln!(
        writer,
        "\nитого: {} (изменено {changed}, только слева {only_left}, только справа {only_right})",
        diffs.len(),
    )?;

    Ok(())
}

pub fn print_error(
    writer: &mut impl std::io::Write,
    err: &dyn std::error::Error,
) -> std::io::Result<()> {
    writeln!(writer, "ошибка: {err}")?;

    let mut source = err.source();
    while let Some(cause) = source {
        writeln!(writer, "  причина: {cause}")?;
        source = cause.source();
    }

    Ok(())
}
