# env_samp

[![Language](https://img.shields.io/badge/Rust-2024%20edition-orange)]()
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Windows-blue)]()
[![Architecture](https://img.shields.io/badge/arch-i686%20(32--bit)-lightgrey)]()

Plugin SA:MP escrito em Rust que carrega variaveis de ambiente a partir de um arquivo `.env` durante o startup do servidor e as expoe para scripts Pawn por meio de uma unica native tipada.

Projetado para eliminar credenciais hardcoded no codigo Pawn, mantendo dados sensiveis fora do versionamento e centralizados em um unico arquivo de configuracao.

## Autor

**NullSablex** - Desenvolvimento e manutencao

## Caracteristicas

- Leitura unica do `.env` no startup (sem overhead em runtime)
- Native unica `Env()` com suporte a tipos: string, int, float e bool
- Parser robusto com suporte a aspas simples, duplas, escapes e comentarios
- Formato flexivel: `CHAVE=VALOR` e `CHAVE = VALOR` sao equivalentes
- Estado global thread-safe via `OnceLock`
- Protecao contra BOM UTF-8 (comum em arquivos criados no Windows)
- Truncacao segura em fronteira de caractere (previne corrupcao UTF-8)
- Limite de 1 MB para prevenir uso indevido do arquivo `.env`
- Valores nunca expostos em logs

## Natives

### Env

```pawn
native bool:Env(const key[], dest[], type = ENV_STRING, dest_len = sizeof(dest));
```

Busca o valor associado a `key` no `.env` e escreve em `dest` no tipo solicitado.

**Tipos disponiveis:**

| Constante    | Valor | Comportamento |
|-------------|-------|---------------|
| `ENV_STRING` | `0`   | Copia a string para `dest` (padrao) |
| `ENV_INT`    | `1`   | Converte para inteiro e escreve em `dest` |
| `ENV_FLOAT`  | `2`   | Converte para float e escreve em `dest` |
| `ENV_BOOL`   | `3`   | Converte para bool (`0`/`1`) e escreve em `dest` |

**Retorno:** `true` se a chave foi encontrada e o valor convertido com sucesso, `false` caso contrario.

**Valores bool reconhecidos:** `true`, `1`, `yes`, `on` / `false`, `0`, `no`, `off`.

### EnvCount

```pawn
native EnvCount();
```

Retorna o total de variaveis carregadas do `.env`.

## Instalacao

1. Copie o plugin para a pasta `plugins/` do servidor:
   - **Linux:** `env_samp.so`
   - **Windows:** `env_samp.dll`

2. Adicione ao `server.cfg`:

   ```
   # Linux
   plugins env_samp.so

   # Windows
   plugins env_samp.dll
   ```

3. Copie `pawn/include/env_samp.inc` para a pasta de includes do compilador Pawn.

4. Crie o arquivo `.env` na raiz do servidor (mesmo diretorio do executavel):

   ```env
   MYSQL_HOST = 127.0.0.1
   MYSQL_PORT = 3306
   MYSQL_USER = samp_user
   MYSQL_PASS = senha_segura
   MYSQL_DB   = meu_banco
   APP_DEBUG  = false
   TICK_RATE  = 0.5
   ```

## Exemplo de uso

```pawn
#include <a_samp>
#include <env_samp>

public OnGameModeInit()
{
    printf("Variaveis carregadas: %d", EnvCount());

    // String (tipo padrao, basta omitir o terceiro argumento)
    new host[64];
    Env("MYSQL_HOST", host);

    // Integer
    new port;
    Env("MYSQL_PORT", port, ENV_INT);

    // Float
    new Float:rate;
    Env("TICK_RATE", rate, ENV_FLOAT);

    // Bool
    new bool:debug;
    Env("APP_DEBUG", debug, ENV_BOOL);

    printf("[DB] %s:%d db_debug=%d tick=%f", host, port, _:debug, rate);
    return 1;
}
```

## Formato do .env

| Formato | Exemplo | Resultado |
|---|---|---|
| Basico | `KEY=value` | `value` |
| Com espacos | `KEY = value` | `value` |
| Aspas simples | `KEY='val\nue'` | `val\nue` (literal) |
| Aspas duplas | `KEY="val\nue"` | `val` + quebra de linha + `ue` |
| Comentario inline | `KEY=value # comment` | `value` |
| Hash colado | `KEY=val#ue` | `val#ue` |
| Valor vazio | `KEY=` | (string vazia) |
| Chave duplicada | ultima ocorrencia vence | |

**Ignorados:** linhas vazias, linhas iniciadas com `#`, linhas com prefixo `export`.

**Escapes em aspas duplas:** `\\`, `\"`, `\n`, `\r`, `\t`.

## Build

Requer Rust (stable) com os targets de 32-bit instalados:

```bash
# Build completo (Linux + Windows)
./scripts/build.sh

# Build manual para um unico target
cargo build --release --target i686-unknown-linux-gnu
cargo build --release --target i686-pc-windows-gnu
```

Artefatos gerados em `dist/`:

```
dist/
  env_samp.so        # Linux plugin
  env_samp.so.sha256
  env_samp.dll       # Windows plugin
  env_samp.dll.sha256
```

## Seguranca

- O `.env` e lido **uma unica vez** no startup. Nao ha releitura em runtime.
- **Valores nunca sao expostos em logs.** Apenas a quantidade de variaveis carregadas e registrada.
- Arquivos acima de 1 MB sao rejeitados automaticamente.
- Adicione `.env` ao `.gitignore`. **Nunca versione credenciais.**
- Use `.env.example` como template publico sem valores reais.

## Estrutura do projeto

```
env_samp/
  src/
    lib.rs          Entry point, native e lifecycle do plugin
    dotenv.rs       Parser .env com suite de testes
    state.rs        Estado global thread-safe (OnceLock)
    log.rs          Logging e banner de inicializacao
  pawn/
    include/
      env_samp.inc  Include com native e constantes de tipo
  examples/
    example.pwn    Exemplo de uso em gamemode
  scripts/
    build.sh        Cross-compilation Linux + Windows 32-bit
  build.rs          Build script para timestamp de compilacao
  .env.example      Template de variaveis
```

## Licenca

Este projeto e distribuido sob a [GNU General Public License v3.0](LICENSE).

Voce pode usar, modificar e redistribuir livremente, desde que qualquer trabalho derivado tambem seja distribuido sob a mesma licenca com o codigo-fonte disponivel.
