<script lang="ts">
    export let icon: string;
    export let alt: string | null = null;
    export let onClick: () => void = () => {};
</script>

<div>
    <button on:mousedown={() => onClick()}>
        <img src="icons/{icon}.svg" {alt} draggable="false" />
    </button>
</div>

<style>
    button {
        background-color: transparent;
        border: none;
        box-shadow: none;
        cursor: pointer;
        padding: 0.3rem;
        border: 1px solid var(--theme-border-inactive);
        transition: cubic-bezier(0.175, 0.885, 0.32, 1.275) 0.2s;
        display: flex;
    }

    img {
        height: 2rem;
        object-fit: cover;
    }

    button:hover {
        border: 1px solid var(--theme-border-active);
        background-color: var(--theme-surface);
        scale: 1.1;
    }

    button:active {
        scale: 0.98;
        rotate: 5deg;
        background-color: var(--theme-container);
    }

    @media (prefers-color-scheme: dark) {
        img {
            filter: invert();
        }
    }
</style>
