<script lang="ts">
    import { process, window } from "@tauri-apps/api";

    function minimize() {
        window.appWindow.minimize();
    }

    function close() {
        process.exit(0);
    }
</script>

<div id="titlebar" class="animated" data-tauri-drag-region>
    <h1 data-tauri-drag-region>OctoVPN</h1>
    <div id="buttons">
        <button on:click={minimize}><img src="icons/minimize.svg" alt="Exit" draggable="false"></button>
        <button on:click={close}><img src="icons/close.svg" alt="Exit" draggable="false" /></button>
    </div>
</div>

<style>
    #titlebar {
        position: absolute;
        top: 0;
        left: 0;
        
        width: 100%;

        border: 1px solid var(--theme-border-inactive);
        background-color: var(--theme-surface);

        display: flex;
        justify-content: space-between;
    }
    
    #buttons {
        margin: 0.2rem 0.5rem;
    }

    #buttons:hover {
        border: none;
    }

    div:hover {
        border: 1px solid var(--theme-border-active);
    }

    h1 {
        margin: 0.5rem 1rem;
        font-size: 1rem;
    }

    img {
        width: 100%;
        height: 100%;
    }

    button {
        width: 2rem;
        height: 2rem;
        padding: .25rem;
    }

    @media (prefers-color-scheme: dark) {
        img {
            filter: invert();
        }
    }
</style>
