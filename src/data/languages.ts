/** Catálogo de lenguajes. Añade aquí los lenguajes nuevos. */
export type LanguageInfo = { id:string; name:string; description:string; accent:string; icon:string; status:'available'|'coming-soon' };
export const languages: LanguageInfo[] = [
 {id:'rust',name:'Rust',description:'Seguridad de memoria, rendimiento y sistemas modernos.',accent:'orange',icon:'🦀',status:'available'},
 {id:'python',name:'Python',description:'Automatización, datos y desarrollo con una sintaxis clara.',accent:'blue',icon:'🐍',status:'coming-soon'},
 {id:'javascript',name:'JavaScript',description:'La base interactiva de la web moderna.',accent:'yellow',icon:'🟨',status:'coming-soon'},
 {id:'go',name:'Go',description:'Servicios y herramientas concurrentes con simplicidad.',accent:'cyan',icon:'🐹',status:'coming-soon'}
];
