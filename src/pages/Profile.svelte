<script lang="ts">
    import { account as storeAccount } from '../stores';
    import type { Account } from '../api/structs';

    let acc: Account | undefined = undefined;
    storeAccount.subscribe((value) => {
        acc = value as Account;
    });
</script>

<div class="profile animated">
    <h3>Profile</h3>

    {#if acc != undefined}
        <div class="section">
            <h3 class="subheader">Personal information</h3>
            <h5>{acc.firstName} {acc.lastName}</h5>
            <h6>{acc.email}</h6>
        </div>

        <div>
            <h3 class="subheader">User type</h3>
            <h5>{acc.userType}</h5>
        </div>
    {:else}
        <p>Loading data...</p>
    {/if}
</div>

<style>
    .profile {
        display: flex;
        flex-direction: column;
        gap: 0rem;
        padding: 1rem;
        border-radius: 1rem;

        pointer-events: all;

        border: 1px solid var(--theme-border-inactive);
    }

    .section {
        margin: 0 0 1.5rem 0;
    }

    .subheader {
        margin: 0;
        font-size: large;
    }

    h5,
    h6 {
        margin: 0;
        color: var(--theme-primary);
        font-weight: normal;
        font-size: 1rem;
    }

    h6 {
        opacity: 0.75;
    }

    .profile:hover {
        background-color: var(--theme-surface);
        border: 1px solid var(--theme-border-active);
    }
</style>
