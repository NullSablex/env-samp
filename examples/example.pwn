#include <a_samp>
#include <env_samp>

main() {}

public OnGameModeInit()
{
    printf("[env_samp] Variables loaded: %d", EnvCount());

    // String (default type)
    new host[64];
    if (Env("MYSQL_HOST", host))
    {
        printf("[env_samp] MYSQL_HOST=%s", host);
    }

    // Integer
    new port;
    if (Env("MYSQL_PORT", port, ENV_INT))
    {
        printf("[env_samp] MYSQL_PORT=%d", port);
    }

    // Float
    new Float:rate;
    if (Env("TICK_RATE", rate, ENV_FLOAT))
    {
        printf("[env_samp] TICK_RATE=%f", rate);
    }

    // Boolean
    new bool:debug;
    if (Env("APP_DEBUG", debug, ENV_BOOL))
    {
        printf("[env_samp] APP_DEBUG=%d", _:debug);
    }

    return 1;
}
