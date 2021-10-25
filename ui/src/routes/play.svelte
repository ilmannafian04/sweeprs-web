<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';

  import util from '$lib/util';
  import socket from '$lib/wsStore';

  let code = $page.query.get('code');
  onMount(() => {
    if (code === null || code === '') {
      code = util.generateCode(4);
      const newUrl = util.addCodeToUrl(code);
      window.history.pushState({ path: newUrl }, '', newUrl);
    }
    socket.set(new WebSocket(`ws://127.0.0.1:8000/${code}`));
    $socket.onmessage = (e) => {
      console.log(e);
    };
  });

  const clickHandler = () => {
    $socket.send('asd');
  };
</script>

<h1>asd</h1>
<button on:click={clickHandler}>Send</button>
