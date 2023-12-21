<script>
    import { config, status } from '../../stores';
    import { ovpn_status } from '../../stores';

    import { Protocol, Status, statusToString } from '../../structs';

    $: ovpn = $ovpn_status != null;
    $: connectedFor = $ovpn_status
        ? new Date((new Date().getTime() / 1000 - $ovpn_status.start) * 1000)
              .toISOString()
              .substring(11, 19)
        : undefined;
</script>

{#if ovpn}
    <p>
        Status: <b>{$ovpn_status?.state.replace('SUCCESS', 'CONNECTED').toLowerCase()}</b> • public
        IP: <b>{$ovpn_status?.remote_ip}</b> • connected for: {connectedFor}
    </p>
{:else}
    <p>Status: <b>{statusToString($status).toLowerCase()}</b>
    {#if $config.protocol == Protocol.OpenVPN && $status == Status.CONNECTED}
        • public IP: <b>not assigned</b> (waiting for server...)
    {/if}
    </p>
{/if}

<style>
    p {
        margin: 0;
    }
</style>
