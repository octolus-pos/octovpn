<script lang="ts">
    import { authToken, config as config, loggedIn } from '../stores';
    import Checkbox from '../lib/Checkbox.svelte';
    import { Protocol } from '../structs';
    import { get } from 'svelte/store';

    async function logout() {
        config.update((value) => {
            value.credentials = null;
            return value;
        });

        loggedIn.set(false);
        authToken.set(null);
    }
</script>

<div id="settings" class="settings animated">
    <h3>Settings</h3>
    <h4>Discord</h4>
    <Checkbox id="discord-rpc" label="Enable Rich Presence" bind:checked={$config.discordRPC} />
    <!-- <Checkbox id="openvpn" label="OpenVPN (legacy)" bind:checked={openVPN} /> -->
    
    <h4>Protocol</h4>
    <select class="animated" bind:value={$config.protocol}>
        <option value={Protocol.OpenVPN}>OpenVPN</option>
        <option value={Protocol.WireGuard}>WireGuard</option>
    </select>

    <h4>Account</h4>
    <button class="animated" on:click={logout}>Log out</button>
</div>

<style>
    select {
        padding: 0.5rem;
        border-radius: 0.5rem;
        border: 1px solid var(--theme-border-inactive);
        background-color: var(--theme-surface);
        color: var(--theme-foreground);
    }

    select:hover {
        border: 1px solid var(--theme-border-active);
    }

    .settings {
        display: flex;
        flex-direction: column;
        gap: 0rem;
        padding: 1rem;
        border-radius: 1rem;

        pointer-events: all;

        border: 1px solid var(--theme-border-inactive);
    }

    .settings:hover {
        background-color: var(--theme-surface);
        border: 1px solid var(--theme-border-active);
    }

    button {
        cursor: pointer;

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

    h3 {
        margin-bottom: 0.5rem;
    }
</style>
