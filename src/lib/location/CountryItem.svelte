<script lang="ts">
    import type { Location } from '../../api/structs';
    import { search } from '../../stores';
    import CityItem from './CityItem.svelte';

    let showCities: boolean = true;
    export let country: [string, Set<Location>];

    let country_code = (country[1].values().next().value as Location).country_code.toLowerCase();

    search.subscribe(() => {
        showCities = true;
    });
</script>

<div class="animated" id="location">
    <button on:click={() => (showCities = !showCities)}>{country[0]}</button>
    {#if showCities}
        {#each country[1] as location}
            <!-- TODO: fix once country codes are implemented in the API -->
            <CityItem loc={location} />
        {/each}
    {/if}
</div>

<style>
    button {
        display: flex;
        box-shadow: none;
        font-size: large;
        font-weight: 500;
        padding: 0.5rem;
    }

    #location {
        width: 100%;

        display: flex;
        flex-direction: column;

        box-shadow: none;
        border-radius: 0.5rem;

        font-weight: normal;
    }
</style>
