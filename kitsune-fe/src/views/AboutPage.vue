<template>
  <div class="about-page">
    <h1 class="title">
      Kitsune hosted on
      <router-link to="/">
        {{ instanceData?.domain }}
      </router-link>
    </h1>

    <fieldset>
      <legend>Instance description</legend>

      <!-- This is chosen by the administrator. It's fine. -->
      <!-- eslint-disable-next-line vue/no-v-html -->
      <span v-html="instanceData?.description" />
    </fieldset>
    <GenericFooter />
  </div>
</template>

<script setup lang="ts">
  import GenericFooter from '../components/GenericFooter.vue';
  import { useInstanceInfo } from '../graphql/instance-info';

  const instanceData = useInstanceInfo();
</script>

<style scoped lang="scss">
  h1 {
    font-size: 3em;
    line-height: 1.1;
  }

  .about-page {
    max-width: 80ch;
    margin: auto;

    & .title {
      text-align: center;
    }
  }
</style>
