<script setup>
import { ref } from 'vue';
import navbar from "../components/navbar.vue";
import bookList from "../components/bookList.vue";
import modal from "../components/modal.vue"

const isModalOpen = ref(false);
</script>

<template>
    <nav>
        <navbar>
            <!-- Camera button -->
            <div class="navbar-button" @click="isModalOpen = true">
                <img src="/images/camera_icon.svg" alt="Icône de caméra">
            </div>
            <!-- Search button -->
            <div class="navbar-button search-button">
                <input type="search" placeholder="Rechercher un livre">
                <img src="/images/magnifying_glass_icon.svg" alt="Icône de recherche">
            </div>
            <modal :isModalOpen="isModalOpen" @closeModal="isModalOpen = false" />
        </navbar>
    </nav>
    <section>
        <bookList />
    </section>
</template>