<script lang="ts">
    import { process } from '@tauri-apps/api';
    import { page } from '../../stores';
    import NavBarItem from './NavBarItem.svelte';
</script>

<div class="navbar animated" data-tauri-drag-region>
    <NavBarItem icon="location" alt="locations" onClick={() => page.set('locations')} />
    <div class="bottom-items">
        <NavBarItem icon="profile" alt="profile" onClick={() => page.set('profile')} />
        <NavBarItem icon="settings" alt="settings" onClick={() => page.set('settings')} />
        <NavBarItem icon="close" alt="close" onClick={() => process.exit(0)} />
    </div>
</div>

<style>
    .bottom-items {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: space-between;
        column-gap: 0rem;
        row-gap: 0.5rem;
    }

    .navbar {
        height: 100%;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: space-between;
        column-gap: 0rem;
        row-gap: 1.5rem;
        padding: 1rem;
        border-radius: 1rem;

        border: 1px solid var(--theme-border-inactive);
        /* background-color: var(--theme-surface); */
    }

    .navbar:hover {
        background-color: var(--theme-surface);
        border: 1px solid var(--theme-border-active);
    }
</style>
