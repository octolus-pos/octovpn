<script lang="ts">
    export let id: string;
    export let label: string;
    export let checked: boolean;
    export let onClick: () => void = () => {};
</script>

<div class="checkbox-wrapper-1">
    <input
        {id}
        class="substituted"
        type="checkbox"
        bind:checked
        aria-hidden="true"
        on:click={() => onClick()}
    />
    <label for={id}>{label}</label>
</div>

<style>
    .checkbox-wrapper-1 *,
    .checkbox-wrapper-1 ::after,
    .checkbox-wrapper-1 ::before {
        box-sizing: border-box;
    }
    .checkbox-wrapper-1 [type='checkbox'].substituted {
        visibility: hidden;
        margin: -1.15em;
        width: 0;
        height: 0;
        display: inline;
        -webkit-appearance: none;
        -moz-appearance: none;
        appearance: none;
    }
    .checkbox-wrapper-1 [type='checkbox'].substituted + label:before {
        content: '';
        display: inline-block;
        vertical-align: top;
        height: 1.15em;
        width: 1.15em;
        margin-right: 0.6em;
        margin-top: 0.15rem;
        color: rgba(0, 0, 0, 0.275);
        border: solid 0.06em;
        box-shadow: 0 0 0.04em, 0 0.06em 0.16em -0.03em inset, 0 0 0 0.07em transparent inset;
        border-radius: 0.2em;
        background: url('data:image/svg+xml;charset=UTF-8,<svg xmlns="http://www.w3.org/2000/svg" version="1.1" xml:space="preserve" fill="white" viewBox="0 0 9 9"><rect x="0" y="4.3" transform="matrix(-0.707 -0.7072 0.7072 -0.707 0.5891 10.4702)" width="4.3" height="1.6" /><rect x="2.2" y="2.9" transform="matrix(-0.7071 0.7071 -0.7071 -0.7071 12.1877 2.9833)" width="6.1" height="1.7" /></svg>')
                no-repeat center,
            var(--theme-surface);
        background-size: 0;
        border: 1px solid var(--theme-border-inactive);
        will-change: color, border, background, background-size, box-shadow;
        transform: translate3d(0, 0, 0);
        transition: color 0.1s, border 0.1s, background 0.15s, box-shadow 0.1s;
    }
    .checkbox-wrapper-1 [type='checkbox'].substituted:enabled:active + label:before,
    .checkbox-wrapper-1 [type='checkbox'].substituted:enabled + label:active:before {
        box-shadow: 0 0 0.04em, 0 0.06em 0.16em -0.03em transparent inset,
            0 0 0 0.07em rgba(0, 0, 0, 0.1) inset;
        background-color: var(--theme-tertiary);
    }
    .checkbox-wrapper-1 [type='checkbox'].substituted:checked + label:before {
        border: 1px solid var(--theme-border-active);
        background-color: var(--theme-primary);
        background-size: 0.75em;
        color: rgba(0, 0, 0, 0.075);
    }
    .checkbox-wrapper-1 [type='checkbox'].substituted:checked:enabled:active + label:before,
    .checkbox-wrapper-1 [type='checkbox'].substituted:checked:enabled + label:active:before {
        border: 1px solid var(--theme-border-active);
        background-color: var(--theme-secondary);
        color: rgba(0, 0, 0, 0.275);
    }
    .checkbox-wrapper-1 [type='checkbox'].substituted:focus + label:before {
        box-shadow: 0 0 0.04em, 0 0.06em 0.16em -0.03em transparent inset,
            0 0 0 0.07em rgba(0, 0, 0, 0.1) inset, 0 0 0 3.3px rgba(65, 159, 255, 0.55),
            0 0 0 5px rgba(65, 159, 255, 0.3);
    }
    .checkbox-wrapper-1 [type='checkbox'].substituted:focus:active + label:before,
    .checkbox-wrapper-1 [type='checkbox'].substituted:focus + label:active:before {
        box-shadow: 0 0 0.04em, 0 0.06em 0.16em -0.03em transparent inset,
            0 0 0 0.07em rgba(0, 0, 0, 0.1) inset, 0 0 0 3.3px rgba(65, 159, 255, 0.55),
            0 0 0 5px rgba(65, 159, 255, 0.3);
    }
    .checkbox-wrapper-1 [type='checkbox'].substituted:disabled + label:before {
        opacity: 0.5;
    }
</style>
