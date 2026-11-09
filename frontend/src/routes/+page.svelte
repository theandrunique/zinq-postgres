<script lang="ts">
  import { appVersion, versionLabel } from "#lib/version.ts";
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>
<span>{versionLabel} ({appVersion.commit})</span>
