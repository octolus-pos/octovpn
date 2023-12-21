<script lang="ts">
    import { invoke } from '@tauri-apps/api';
    import Locations from './pages/Locations.svelte';
    import NavBar from './lib/components/NavBar.svelte';
    import Settings from './pages/Settings.svelte';
    import { get } from 'svelte/store';
    import { authToken, location, page as storePage, config, status, loggingIn, loggedIn } from './stores';
    import { openvpn, wireguard } from './api/client';
    import TitleBar from './lib/components/TitleBar.svelte';
    import Profile from './pages/Profile.svelte';
    import { onMount } from 'svelte';
    import { Protocol, type Config, Status as eStatus, statusToString } from './structs';
    import type { APIResponse, ConfigResponse } from './api/structs';
    import Login from './pages/Login.svelte';
    import Status from './lib/components/Status.svelte';

    let buttonDisabled = get(location) === null;
    // location.subscribe((value) => {
    //   buttonDisabled = value === null && $status == eStatus.DISCONNECTED;
    // })

    $: buttonText = $status == eStatus.DISCONNECTED ? 'Connect' : 'Disconnect';
    $: nextState = $status == eStatus.DISCONNECTED;
    $: $location, (buttonDisabled = shouldBeDisabled($status));

    function shouldBeDisabled(status: eStatus) {
        switch (status) {
            case eStatus.CONNECTED:
                return false; // Always let the user disconnect
            case eStatus.CONNECTING:
                return true; // Don't let them fuck with the connection
            case eStatus.DISCONNECTING:
                return true; // Same here
            case eStatus.DISCONNECTED:
                return get(location) == null; // Only let them connect if they have a location set
        }
    }

    storePage.subscribe((pg) => {
        let currentPage = document.getElementById(pg)!;
        checkPagesVisibility(currentPage);
    });

    async function connect() {
        // Simply disconnect
        if (!nextState) {
            invoke<boolean>('toggle_connection', { state: nextState });
            return;
        }

        // TODO: failsafe this and allow for protocol selection
        let configuration = get(config);
        if (!configuration) {
            invoke<Config>('load_config').then((res) => {
                config.set(res);
            });
        }
        configuration = get(config)!;

        let loc = get(location)!;

        if (configuration.credentials) {
            var res: APIResponse<ConfigResponse>;

            // if they can see this instead of the login screen, they must have a token
            if (configuration.protocol == Protocol.OpenVPN) {
                // TODO: add protocol switching option in config
                res = await openvpn($authToken!, loc, 'udp');
            } else {
                res = await wireguard($authToken!, loc);
            }

            console.log(res);

            if (res.success) {
                invoke<boolean>('toggle_connection', {
                    state: nextState,
                    config: res.data!.config,
                    credentials: configuration!.credentials,
                });
            } else {
                // TODO: show error to user
            }
        }
    }

    function togglePage(page: HTMLElement, visible: boolean) {
        page.style.opacity = visible ? '1' : '0';
        page.style.visibility = visible ? 'visible' : 'hidden';
    }

    function checkPagesVisibility(currentPage: HTMLElement) {
        let pages = document.getElementsByClassName('page');

        for (let i = 0; i < pages.length; i++) {
            let page = pages[i] as HTMLElement;
            let isCurrentPage = page.id == currentPage.id;

            togglePage(page, isCurrentPage);
        }
    }

    onMount(async () => {
        let currentPage = document.getElementById(get(storePage))!;
        checkPagesVisibility(currentPage);
    });
</script>

<main>
    <!-- DEBUG: remove `!` -->
    {#if $loggedIn}
        <TitleBar />

        <div id="locations" class="page animated"><Locations /></div>
        <div id="profile" class="page animated" style="opacity: 0; visibility: hidden;">
            <Profile />
        </div>
        <div id="settings" class="page animated" style="opacity: 0; visibility: hidden;">
            <Settings />
        </div>

        <div class="navbar">
            <NavBar />
        </div>

        <div class="main-btn animated">
            <button
                id="connect-btn"
                class="animated"
                class:disabled={buttonDisabled}
                on:click={() => {
                    connect();
                }}
            >
                {buttonText}
            </button>
        </div>

        <div class="status">
            <Status />
        </div>
    <!-- DEBUG: remove `!` -->
    {:else if $loggingIn} 
        <div id="logging-in" class="animated">
            <h1>Logging in</h1>
            <p>Please wait...</p>
        </div>
    {:else}
        <Login />
    {/if}
</main>

<style>
    #logging-in {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: 100vh;
        width: 90vw;
    }

    .navbar {
        grid-row-start: 1;
        grid-row-end: 3;

        margin-bottom: 1rem;
        margin-top: 1.5rem;
    }

    .status {
        margin-top: 0.5rem;
        grid-column: 2 / -1;
    }

    .page {
        pointer-events: none;
        grid-row: 1;
        grid-column: 2;
        border-radius: 1rem;

        margin-top: 1.5rem;
    }

    .main-btn {
        grid-column: 2;
        grid-row: 2;
        margin-top: 1rem;
    }

    .disabled {
        pointer-events: none;
        opacity: 0.5;
        scale: 0.95;
    }

    button {
        width: 100%;

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
