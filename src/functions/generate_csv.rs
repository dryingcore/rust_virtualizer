use std::fs::{OpenOptions};
use std::io::Write;
pub fn write_to_csv(
    full_caller_id: &str,
    operadora: &str,
    saldo: f64,
    status_ativo: &str,
    iccid: &str,
    plano_dados: f64,
    data_ativacao: &str,
) -> std::io::Result<()> {
    let file_path = "linhas.csv";

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)?;

    if file.metadata()?.len() == 0 {
        writeln!(
            file,
            "Linha,Operadora,Saldo (MB),Status,ICCID,Plano de Dados (MB),Data de Ativação"
        )?;
    }

    writeln!(
        file,
        "{},{},{:.2},{},{},{:.0},{}",
        full_caller_id, operadora, saldo, status_ativo, iccid, plano_dados, data_ativacao
    )?;

    Ok(())
}