<script lang="ts">
    import type { Location } from '../../api/structs';
    import { location } from '../../stores';
    // import { Icon } from 'svelte-circle-flags';

    export let loc: Location;
    // export let country_code: string;

    let random_ping = Math.floor(Math.random() * 1000);
</script>

<button
    class="animated"
    id="location"
    on:click={() => {
        location.set(loc);
        // search.set("");
    }}
>
    <div class="left-half animated">
        <span class={`fi fi-${loc.country_code.toLowerCase()}`}></span>
        {loc.city ?? `(${loc.name})`}
    </div>
    <div class="right-half">
        <!-- TODO: calculate real ping -->
        {random_ping}ms
    </div>
</button>

<style>
    .fi {
        scale: 1.5;
        border-radius: 3px;
        height: 100%;
    }

    #location {
        font-weight: normal;

        width: 100%;
        gap: 1rem;

        display: flex;
        justify-content: space-between;
        flex-direction: row;

        box-shadow: none;
        border-radius: 0.5rem;
    }

    .left-half {
        display: flex;
        flex-direction: row;
        gap: 1rem;

        pointer-events: none;
    }

    .right-half {
        color: var(--theme-primary);
        opacity: 0.75;
    }

    button:hover {
        cursor: pointer;
        padding-left: 1.75rem;
        padding-right: 1.75rem;
    }
</style>
