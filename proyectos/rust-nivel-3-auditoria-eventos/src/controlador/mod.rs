use crate::{service, vista};

const DATOS_DEMO: &str = "\
2026-10-27T08:10:02Z|usuario_17|LOGIN|portal|OK
2026-10-27T08:11:14Z|usuario_17|LOGIN|portal|DENIED
2026-10-27T08:11:42Z|usuario_17|RESET_PASSWORD|portal|DENIED
2026-10-27T08:12:02Z|usuario_17|LOGIN|portal|DENIED
2026-10-27T08:14:03Z|admin_2|CHANGE_ROLE|usuario_17|OK
2026-10-27T08:15:03Z|usuario_22|LOGIN|portal|OK
linea malformada
2026-10-27T08:17:03Z|usuario_23|LOGIN|portal|BLOCKED";

pub fn ejecutar() {
    let texto = DATOS_DEMO.to_string();
    let (eventos, invalidas) = service::analizar(&texto);
    vista::imprimir_informe(&eventos, invalidas);
}
