<script lang="ts">
    import { config, location as storeLocation, search as storeSearch } from '../stores';

    import type { Location } from '../api/structs';
    import CountryItem from '../lib/location/CountryItem.svelte';
    import { get } from 'svelte/store';
    import { locations as storeLocations } from '../stores';
    import { Protocol } from '../structs';

    let allLocations = get(storeLocations);
    let allMapped = new Map<string, Set<Location>>();
    let filtered = allMapped;

    function mapLocations(locations: Location[]): Map<string, Set<Location>> {
        let mapped = new Map<string, Set<Location>>();

        locations.forEach((location) => {
            let country = location.country;
            if (country == null) {
                country = 'Unknown';
            }

            if (!mapped.has(country)) {
                mapped.set(country, new Set<Location>());
            }

            mapped.get(country)!.add(location);
        });

        return mapped;
    }

    function filterLocations() {
        let query = $storeSearch.trim().toLowerCase();
        filtered = allMapped;

        if (query.length > 0) {
            filtered = new Map<string, Set<Location>>();
            allMapped.forEach((locations, country) => {
                let filteredLocations = new Set<Location>();
                locations.forEach((location) => {
                    if (location.name.toLowerCase().includes(query)) {
                        filteredLocations.add(location);
                    }
                });

                if (filteredLocations.size > 0) {
                    filtered.set(country, filteredLocations);
                }

                if (country.toLowerCase().includes(query)) {
                    filtered.set(country, locations);
                }
            });
        }
    }

    storeLocations.subscribe((value) => {
        allLocations = value;

        // Filter based on WireGuard
        if ($config.protocol == Protocol.WireGuard) {
            value = value.filter((location) => location.hasWireGuardConfig);
        }

        allMapped = mapLocations(value);
        filterLocations();
    });
</script>

<div id="country-list" class="country-list animated">
    <h3>Location</h3>

    <h4>Current</h4>
    <div class="placeholder">
        {#if $storeLocation != null}
            <!-- TODO: fix once country code is added to API -->
            <span class={`fi fi-${$storeLocation.country_code.toLowerCase()}`}></span>
            <!-- <Icon name={$storeLocation.country_code} height=24 width=24 />  -->
            <b>{$storeLocation.city ?? `(${$storeLocation.name})`}</b>
        {:else}
            Pick a server...
        {/if}
    </div>

    <div class="search-box">
        <h4>Search</h4>
        <textarea
            id="search-box"
            class="animated"
            bind:value={$storeSearch}
            on:input={filterLocations}
        />
    </div>

    <div class="country-menu animated">
        {#each filtered as location}
            <!-- <RegionItem region={region} /> TODO: put back once regions are added -->
            <CountryItem country={location} />
        {/each}
    </div>
</div>

<style>
    .fi {
        margin-left: 1rem;
        scale: 1.5;
        border-radius: 3px;
    }

    h3 {
        margin-bottom: 1rem;
    }

    .placeholder {
        display: flex;
        align-items: center;
        gap: 1rem;
        padding: 0.5rem;

        background-color: var(--theme-surface);
        border: 1px solid var(--theme-border-inactive);
        border-radius: 0.5rem;
    }

    * {
        pointer-events: all;
    }

    .country-list:hover {
        border: 1px solid var(--theme-border-active);
        background-color: var(--theme-surface);
    }

    .country-list {
        gap: 0rem;
        padding: 1rem;
        border-radius: 1rem;

        border: 1px solid var(--theme-border-inactive);
    }

    @media (prefers-color-scheme: dark) {
        .country-list {
            box-shadow: 0px 0px 10px 0px rgba(0, 0, 0, 0.75);
        }
    }

    .country-menu {
        height: 35vh;

        overflow-y: scroll;
        overflow-x: hidden;

        display: flex;
        flex-direction: column;

        border: 1px solid var(--theme-border-inactive);
        border-radius: 0.5rem;
        margin-top: 1rem;
        background-color: var(--theme-background-color);
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
</style>
