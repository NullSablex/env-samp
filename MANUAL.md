# Manual de Uso - env_samp

Documentação completa da API Pawn para o plugin de variaveis de ambiente env_samp.

## Instalacao Basica

### 1. Include no seu gamemode

```pawn
#include <env_samp>
```

O arquivo `env_samp.inc` ja contem as definicoes das natives e constantes de tipo.

### 2. Natives disponiveis

```pawn
native bool:Env(const key[], dest[], type = ENV_STRING, dest_len = sizeof(dest));
native EnvCount();
```

---

## API Reference

### Env()

Busca uma variavel de ambiente do arquivo `.env` e a escreve no buffer `dest` com o tipo especificado.

**Assinatura:**
```pawn
bool:Env(const key[], dest[], type = ENV_STRING, dest_len = sizeof(dest))
```

**Parametros:**

| Parametro | Tipo | Descricao |
|-----------|------|-----------|
| `key` | `const char[]` | Nome da variavel a buscar (case-sensitive) |
| `dest` | Buffer | Local onde o valor sera armazenado |
| `type` | `i32` | Tipo de dado esperado (veja tabela abaixo) |
| `dest_len` | `i32` | Tamanho do buffer (auto-calculado por `sizeof(dest)` por padrao) |

**Tipos suportados:**

| Constante | Valor | Comportamento |
|-----------|-------|---------------|
| `ENV_STRING` | `0` | Copia a string literal para `dest` |
| `ENV_INT` | `1` | Converte para inteiro e escreve em `dest[0]` |
| `ENV_FLOAT` | `2` | Converte para float e escreve em `dest[0]` |
| `ENV_BOOL` | `3` | Converte para booleano (`0` ou `1`) em `dest[0]` |

**Retorno:**

- `true` — Variavel encontrada e valor copiado/convertido com sucesso
- `false` — Variavel nao encontrada (buffer recebe valor padrao/zero)

**Exemplo basico:**

```pawn
new host[64];
if (Env("MYSQL_HOST", host))
{
    printf("Conectando em %s", host);
}
else
{
    printf("Variavel MYSQL_HOST nao definida");
}
```

---

### EnvCount()

Retorna o total de variaveis carregadas do arquivo `.env` no startup.

**Assinatura:**
```pawn
EnvCount()
```

**Retorno:** Inteiro com o total de chaves carregadas.

**Exemplo:**
```pawn
printf("Total de variaveis: %d", EnvCount());
```

---

## Exemplos de Uso

### Exemplo 1: Dimensionamento correto de buffers

Em Pawn, o tamanho do buffer (ex: `[64]`) define quantos **caracteres** podem ser armazenados:

```pawn
// Nomes de host sao pequenos
new host[64];          // 63 caracteres + 1 null terminator
Env("MYSQL_HOST", host);

// Senhas podem ser longas
new pass[128];         // 127 caracteres + 1 null terminator
Env("MYSQL_PASS", pass);

// URLs podem ser muito longas
new api_url[512];      // 511 caracteres + 1 null terminator
Env("API_URL", api_url);
```

**Regra simples:** o tamanho do array = numero de caracteres esperado + 1 (para null terminator).

**Exemplo completo:**

```pawn
new host[64], port, user[32], pass[128], db[32];

// Carregar todas as variaveis
Env("MYSQL_HOST", host);
Env("MYSQL_PORT", port, ENV_INT);
Env("MYSQL_USER", user);
Env("MYSQL_PASS", pass);
Env("MYSQL_DB", db);

printf("[DB] Conectando em %s:%d usuario=%s banco=%s", host, port, user, db);
```

Se a variavel no `.env` for maior que o buffer, ela e **truncada silenciosamente**.

### Exemplo 2: Configuracoes booleanas e float

```pawn
new bool:debug, Float:tick_rate;

Env("APP_DEBUG", debug, ENV_BOOL);
Env("TICK_RATE", tick_rate, ENV_FLOAT);

if (debug)
{
    printf("[CONFIG] Debug ativado, tick rate=%.2f", tick_rate);
}
```

### Exemplo 3: Validacao com valores padrao

```pawn
new port;
if (!Env("SERVER_PORT", port, ENV_INT))
{
    port = 7777; // Valor padrao
    printf("[CONFIG] PORT nao definido, usando 7777");
}
```

### Exemplo 4: Loop por todas as variaveis

```pawn
printf("Total de variaveis carregadas: %d", EnvCount());
// Nota: O plugin nao oferece API para iterar por chave/valor.
// Use EnvCount() apenas para debugging ou diagnosticos.
```

---

## Arquivo `.env` - Formato

> [!NOTE]
> O arquivo `.env` e lido apenas uma vez no startup do servidor. Alteracoes requerem reinicializacao.

### Formato valido

```env
# Comentarios iniciados com #
CHAVE=valor

# Espacos sao trimmed
DB_HOST = 127.0.0.1
DB_PORT = 3306

# Aspas simples (valores literais, sem escapes)
LITERAL='texto com \n literal'

# Aspas duplas (interpreta escapes)
MOTD="Bem-vindo!\nServidor ativo"

# Comentario inline (requer espaco antes de #)
API_KEY=abc123def456 # sua chave aqui

# Hash sem espaco faz parte do valor
HASH_VALUE=abc#def

# Chaves duplicadas: ultima vence
DEBUG=false
DEBUG=true
```

### Escapes suportados em aspas duplas

| Escape | Resultado |
|--------|-----------|
| `\\` | Barra invertida `\` |
| `\"` | Aspas duplas `"` |
| `\n` | Quebra de linha |
| `\r` | Retorno de carro |
| `\t` | Tabulacao |

### Formatos nao suportados

```env
# Nao aceito: prefixo export
export DB_HOST=127.0.0.1

# Nao aceito: valores multiline
MULTILINE="linha1
linha2"

# Nao aceito: expansao de variaveis
EXPANDED=${DB_HOST}:3306
```

---

## Dimensionamento de Buffers

Em Pawn, o tamanho do array determina quantos caracteres **no maximo** podem ser armazenados.

### Formula

```
Tamanho do buffer = Numero maximo de caracteres esperados + 1 (null terminator)
```

**Resumo prático:**
> Sempre some 1 ao tamanho esperado. Se quer armazenar 50 caracteres, use `[51]`. Se quer algo genérico, arredonde para cima e adicione 1.

### Tabela de referencia

Use esta tabela como guia para dimensionar seus buffers:

| Tipo de dado | Tamanho sugerido | Motivo |
|--------------|-----------------|--------|
| Hostname/IP | `[64]` ou `[128]` | IPv4 simples (15 chars), mas hostname pode ser longo |
| Porta (string) | `[8]` ou `[16]` | Porta em string (max 5 numeros) |
| Usuario banco | `[32]` ou `[64]` | Nomes de usuario sao geralmente curtos |
| Senha | `[128]` ou `[256]` | Senhas devem ser longas para seguranca |
| Nome banco | `[64]` | Nomes de banco sao razoavelmente curtos |
| URL/URI | `[512]` ou `[1024]` | URLs podem ser muito longas |
| Token/API Key | `[256]` | Tokens sao geralmente longos |
| Mensagem/String generica | `[256]` a `[512]` | Depende do conteudo |

### Exemplos praticos

```pawn
// Curto
new lang[8];              // "pt_BR" = 5 chars
Env("LANGUAGE", lang);

// Medio
new email[128];           // emails podem ter ~100 chars
Env("ADMIN_EMAIL", email);

// Longo
new api_key[256];         // chaves sao longas
Env("API_SECRET_KEY", api_key);

// Muito longo
new conn_str[512];        // connection strings podem ser complexas
Env("DATABASE_URL", conn_str);
```

### O que acontece se for pequeno demais?

Se o valor no `.env` for **maior** que o buffer:

```pawn
// .env
MYSQL_PASS=SenhaRealmenteLongaDemasiado123!@#

// Codigo - ERRADO, buffer pequeno
new pass[20];
Env("MYSQL_PASS", pass);
// pass contera apenas: "SenhaRealmenteLongaD" (truncado)
```

**Solucao:** aumente o tamanho do buffer

```pawn
// Codigo - CORRETO, buffer adequado
new pass[256];  // Suficiente para qualquer senha
Env("MYSQL_PASS", pass);
```

### Dica de seguranca

Para senhas e tokens, **sempre use buffers generosos**:

```pawn
// Senhas
new db_pass[256];        // Nao poupe tamanho

// API Keys
new api_token[512];      // Chaves modernas sao longas

// Connection Strings
new conn_str[1024];      // Conexoes podem ter query strings complexas
```

---

## Comportamento e Garantias

> [!IMPORTANT]
> Leia isto para evitar erros em producao.

### Leitura unica no startup

- O arquivo `.env` e lido exatamente uma vez quando o servidor inicia.
- **Nao ha releitura em runtime.**
- Se modificar `.env`, **reinicie o servidor.**

### Valores nunca sao logados

- Os valores das variaveis **nunca aparecem em logs ou console** (seguranca).
- Apenas o total de variaveis carregadas e logado.
- Evita exposicao de senhas ou tokens em arquivos de log.

### Tamanho maximo

- Arquivos `.env` com mais de **1 MB sao rejeitados.**
- Se o servidor falhar a carregar, verifique o tamanho.

### Protecao contra corrompcao UTF-8

- Arquivos salvos com BOM UTF-8 (comum no Notepad do Windows) sao tratados corretamente.
- Truncacao de valores e feita em fronteira de caractere, nunca no meio de um caractere multi-byte.

### Conflitos de chaves

- Se a mesma chave aparece multiplas vezes no `.env`, a **ultima ocorrencia vence.**

---

## Diagnosticos

### Variavel nao encontrada?

1. **Confira o nome exacto** — e case-sensitive (`MYSQL_HOST` != `mysql_host`)
2. **Confira o arquivo `.env`** — deve estar na raiz do servidor (mesmo diretorio do executavel)
3. **Reinicie o servidor** — mudar `.env` so funciona apos reinicializacao
4. **Verifique o tipo** — se definiu `Env("PORT", var, ENV_INT)`, a variavel deve ser um inteiro valido

### Conversao falha?

- `ENV_INT` — se o valor nao for um inteiro valido, retorna `false` e `dest[0] = 0`
- `ENV_FLOAT` — se o valor nao for um float valido, retorna `false` e `dest[0] = 0.0`
- `ENV_BOOL` — aceita `true`, `1`, `yes`, `on` (true) ou `false`, `0`, `no`, `off` (false)

### String truncada?

- Se a string no `.env` for maior que o buffer Pawn, ela e truncada automaticamente.
- Use buffers maiores se necessario: `new var[256];` ao inves de `new var[32];`

---

## Boas Praticas

> [!TIP]
> Siga estas orientacoes para codigo robusto em producao.

### 1. Sempre validar retorno

```pawn
// Ruim
Env("DB_HOST", host);

// Bom
if (!Env("DB_HOST", host))
{
    host = "127.0.0.1"; // Padrao
}
```

### 2. Usar template `.env.example`

Mantenha um arquivo `.env.example` com **todas as chaves esperadas** mas sem valores reais:

```env
# .env.example (commit no git)
MYSQL_HOST=
MYSQL_PORT=
MYSQL_USER=
MYSQL_PASS=
APP_DEBUG=false
```

Usuarios fazem uma copia:
```bash
cp .env.example .env
# ... editar .env com valores reais ...
```

### 3. Nao versionem `.env`

Adicione ao `.gitignore`:
```
.env
```

### 4. Carregar variaveis cedo

Carregue todas as variaveis no `OnGameModeInit()` ou `OnFilterScriptInit()`, nao no meio da execucao:

```pawn
public OnGameModeInit()
{
    // Fazer isto aqui
    new host[64];
    Env("MYSQL_HOST", host);

    // Nao aqui (custoso repetir)
    return 1;
}

public SomeCallback()
{
    // Evitar
    // new host[64];
    // Env("MYSQL_HOST", host); // Ja foi carregado acima
}
```

### 5. Documentar variaveis esperadas

No seu `.env.example`, adicione comentarios:

```env
# Banco de dados
MYSQL_HOST=
MYSQL_PORT=3306
MYSQL_USER=
MYSQL_PASS=

# Debug
APP_DEBUG=false
LOG_LEVEL=INFO
```

---

## Troubleshooting

| Problema | Causa | Solucao |
|----------|-------|---------|
| `Env()` retorna `false` para chaves validas | Chave nao definida no `.env` ou `.env` nao existe | Verif ficheiro `.env` e reinicie |
| Variavel com caracteres estranhos | BOM UTF-8 no arquivo | Salve como UTF-8 sem BOM |
| String truncada no meio de acentos | Buffer pequeno demais | Aumente tamanho do buffer |
| Conversao INT/FLOAT falha | Valor nao e numero valido | Valide o `.env` |
| `EnvCount()` retorna 0 | Arquivo `.env` nao existe ou esta vazio | Crie `.env` com variaveis |

---

## Seguranca

> [!WARNING]
> Proteja seus arquivos `.env` como dados sensiveis.

### Do's

- ✅ Mantenha `.env` fora do versionamento (use `.gitignore`)
- ✅ Use permissoes de arquivo restritivas: `chmod 600 .env`
- ✅ Nunca commite `.env` real no repositorio
- ✅ Use `.env.example` como template publico
- ✅ Rotacione senhas regularmente

### Don'ts

- ❌ Nao commet o arquivo `.env` real
- ❌ Nao compartilhe credenciais em Slack/Discord
- ❌ Nao versione chaves de API ou senhas
- ❌ Nao deixe o servidor aberto com `.env` visivelmente sensivel

---

## Limitacoes Conhecidas

- **Nao ha iteracao**: nao e possivel listar todas as chaves, apenas buscar por nome
- **Nao ha releitura**: `.env` e lido uma unica vez no startup
- **Nao ha expansao**: `${VAR}` nao e suportado
- **Nao ha multiline**: quebras de linha dentro de valores nao sao suportadas (use `\n` com aspas duplas)
- **Tamanho maximo de 1 MB**: arquivos maiores sao rejeitados

---

## Versao

Plugin: **v0.1.0**
Ultima atualizacao: 2026-02-17

Para relatorios de bugs ou sugestoes: [GitHub Issues](https://github.com/NullSablex/env_samp/issues)
