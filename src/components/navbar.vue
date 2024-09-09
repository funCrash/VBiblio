<script setup>
</script>

<template>
    <slot></slot>
</template>

<style>
.navbar-button {
    height: 100%;
    width: 100%;
    position: relative;
    z-index: 2;
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: pointer;
}
.navbar-button::before {
    content: "";
    position: absolute;
    z-index: -1;
    inset: 0;
    background: var(--vbiblio-background-color);
    border-radius: 0 0px 40px 40px;
    box-shadow: 0 10px 50px #000;
    transform-origin: 50% 0;
    transition: transform 250ms ease-out;
}
.navbar-button:hover::before {
    transform: scaleY(1.2);
}
.navbar-button img {
    margin-left: auto;
    margin-right: auto;
    height: 90%;
}
.search-button input {
    appearance: none;
    width: 0;
    height: 50%;
    font-size: 2.5em;
    background-color: black !important;
    border: none;
    border-radius: 10px;
    transition: 250ms;
    margin-left: 4%;
}
.search-button:hover input, .search-button input:focus {
    width: 60%;
}
</style>