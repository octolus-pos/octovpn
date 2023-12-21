<script lang="ts">
    import { login } from '../api/client';
    import { Config, Credentials } from '../structs';
    import { authToken, config, loggingIn } from '../stores';
    import { invoke } from '@tauri-apps/api';

    let username = '';
    let password = '';

    async function tryLogin() {
        loggingIn.set(true);

        let credentials = new Credentials(username, password);
        let result = await login(credentials);
        if (result.success) {
            authToken.set(result.data!.token);

            let configuration = await invoke<Config>('load_config');
            configuration.credentials = credentials;
            config.set(configuration);

            invoke('save_config', { config: configuration });
        } else {
            loggingIn.set(false);
        }
    }
</script>

<div id="login">
    <h1>Login</h1>

    <div class="section">
        <h3>Username</h3>
        <textarea id="username-box" class="animated" bind:value={username} />
    </div>

    <div class="section">
        <h3>Password</h3>
        <textarea id="password-box" class="animated" bind:value={password} />
    </div>

    <button class="animated" on:click={tryLogin}>Login</button>
</div>

<style>
    #login {
        display: flex;
        width: 87vw;
        height: 100vh;

        flex-direction: column;
        justify-content: center;
    }

    .section {
        margin-bottom: 1rem;
    }

    h3 {
        font-size: medium;
        margin: 0 0 0.5rem 0;
    }

    textarea {
        box-sizing: border-box;
        width: 100%;
        padding: 1rem 0rem 0.5rem 0.5rem;

        font-size: large;
        white-space: normal;
        text-align: justify;
        line-height: 0.5rem;

        color: var(--theme-foreground);
        background-color: var(--theme-background-color);

        border: 1px solid var(--theme-border-inactive);
        border-radius: 0.5rem;

        resize: none;
    }

    textarea:hover {
        border: 1px solid var(--theme-border-active);
    }

    textarea:focus {
        outline: none !important;
        border: 1px solid var(--theme-secondary);
    }

    button {
        margin-top: 3rem;
        cursor: pointer;
        font-size: larger;

        background-color: var(--theme-surface);
        border: 1px solid var(--theme-border-inactive);
        border-radius: 0.5rem;
    }

    button:hover {
        background-color: var(--theme-surface);
        border: 1px solid var(--theme-border-active);
    }

    button:active {
        scale: 0.99;
        translate: 0 0.25rem;
        background-color: var(--theme-surface-active);
    }
</style>
