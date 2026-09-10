use cfgdrift::Difference;

pub fn print_diff(writer: &mut impl std::io::Write, diffs: &[Difference]) -> std::io::Result<()> {
    if diffs.is_empty() {
        writeln!(writer, "Конфигурации идентичны")?;
        return Ok(());
    }

    writeln!(writer, "Найдены различия:")?;
    writeln!(writer, "  ~  значение изменилось (было → стало)")?;
    writeln!(writer, "  -  ключ есть только в первом файле")?;
    writeln!(writer, "  +  ключ есть только во втором файле")?;
    writeln!(writer)?;

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
                writeln!(writer, "- {key}: {value}")?;
                only_left += 1;
            }
            Difference::OnlyInRight { key, value } => {
                writeln!(writer, "+ {key}: {value}")?;
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
    writeln!(writer, "ошибка: {err}")?;

    let mut source = err.source();
    while let Some(cause) = source {
        writeln!(writer, "  причина: {cause}")?;
        source = cause.source();
    }

    Ok(())
}
