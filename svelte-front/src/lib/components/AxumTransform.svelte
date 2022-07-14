<script lang="ts">
  import { axumTransformClient } from "../config";

  import { displayBanner } from "../util";
  let content = "";

  async function submit() {
    try {
      const response = await axumTransformClient.rootPost({
        transformation: "capitalize",
        data: content,
      });
      displayBanner(response.data.data);
    } catch (err) {
      displayBanner("Error on capitalize", err);
    }
  }
</script>

<div>
  <h2>Axum transform</h2>
  <p>Send a message and get the response</p>
  <form on:submit|preventDefault={submit}>
    <textarea bind:value={content} />
    <input type="submit" value="Enviar" />
  </form>
</div>
