<template>
  <nav class="nav-bar">
    <div class="nav-bar-links">
      <template v-for="(details, route) in links" :key="route">
        <NavBarLink :to="route" :icon="details.icon" :detail="details.detail" />
      </template>
    </div>
    <div class="nav-bar-profile">
      <div class="nav-bar-element profile-menu-button">
        <!-- Without this weird double quote stuff Vite would have tried to do some fucked up shit -->
        <img :src="'/public/assets/default-avatar.png'" />
      </div>
      <div class="nav-bar-element">
        <font-awesome-icon
          class="icon create-status"
          icon="fa-pen-to-square fa-solid"
        />
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
  import NavBarLink from './NavBarLink.vue';

  type RouteInfo = {
    icon: string;
    detail: string;
  };

  const links: Record<string, RouteInfo> = {
    '/timeline/home': {
      icon: 'fa-house fa-solid',
      detail: 'Home',
    },
    '/notifications': {
      icon: 'fa-bell fa-solid',
      detail: 'Notification',
    },
    '/messages': {
      icon: 'fa-envelope fa-solid',
      detail: 'Messages',
    },
    '/timeline/local': {
      icon: 'fa-users fa-solid',
      detail: 'Local',
    },
    '/timeline/federated': {
      icon: 'fa-globe-europe fa-solid',
      detail: 'Federated',
    },
  };
</script>

<style scoped lang="scss">
  @use '../styles/colours' as *;
  @use '../styles/mixins' as *;

  .nav-bar {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    background-color: $dark2;
    padding: 0 25px;
    padding-top: 5px;
    margin-bottom: 100px;
    display: flex;
    justify-content: space-between;
    align-items: center;

    @include only-on-mobile {
      padding: 0;
      padding-top: 5px;

      & .detail {
        display: none;
      }

      & .icon {
        margin-right: 0px;
      }
    }

    &-profile {
      display: flex;
      gap: 10px;

      .create-status {
        height: 25px;
      }

      .profile-menu-button {
        border-radius: 4px;
        display: flex;
        align-items: center;

        img {
          height: 30px;
        }
      }
    }
  }
</style>
