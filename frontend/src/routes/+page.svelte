<script lang="ts">
  import { appVersion, versionLabel } from "#lib/version.js";
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

<footer class="mt-8 flex items-center gap-2 text-sm text-gray-500">
  <span>{versionLabel}</span>
  {#if appVersion.commit}
    <span title="Commit">{appVersion.commit}</span>
  {/if}
  {#if appVersion.dev}
    <span class="rounded bg-amber-100 px-1.5 py-0.5 text-xs font-semibold text-amber-800">DEV</span>
  {/if}
</footer>
