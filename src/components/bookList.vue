<script setup>
import { ref, onMounted } from "vue";
import  db from "../shared/SQL_API";
import { useRouter } from "vue-router";

const router = useRouter();

const books = ref(undefined);
onMounted(async () => {
    books.value = await db.select("SELECT PK_id, title FROM book;");
});
</script>


<template>
    <ul>
        <li v-for="(book, index) in books" :key="index" class="book-item" @click="router.push(`/book/${book.PK_id}`)">
            {{ book.title }}
        </li>
    </ul>
</template>

<style scoped>
ul {
    margin-top: 20px;
    display: flex;
    justify-content: space-around;
    align-items: stretch;
    flex-wrap: wrap;
}
.book-item {
    background: rgba(var(--vbiblio-orange), .8);
    width: calc(100vw/3 - 15px);
    border-radius: 15px;
    margin-bottom: 20px;
    overflow: hidden;
    text-align: center;
    border: 10px double transparent;
    transition: all 350ms cubic-bezier(.26,-0.54,.57,1.61);
}
.book-item:hover {
    border: 10px double #fff;
}
</style>