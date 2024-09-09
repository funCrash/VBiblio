<script setup>
import { RouterView } from "vue-router";
</script>

<template>
    <RouterView />
</template>

<!-- Global styling -->
<style>
nav {
    height: 10vh;
    display: flex;
}

* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
    color: #fff;
    font-family: "Finalnew", "sans-serif";
}
:root {
    --vbiblio-blue: 35, 165, 255;
    --vbiblio-purple: 81, 56, 137;
    --vbiblio-orange: 224, 132, 2;
    --vbiblio-background-color: #707070;
}
html, body, #app {
    height: 100%;
}
a {
    text-decoration: none;
}
li {
    list-style-type: none;
}
body {
    background: url("/images/logo.png") no-repeat 50% 50% fixed,
                var(--vbiblio-background-color);
    background-size: contain;
}
@font-face {
    font-family: "Finalnew";
    src: url("../fonts/Finalnew.ttf");
}
</style>