<script setup>
import navbar from "../components/navbar.vue";
import db from "../shared/SQL_API";
import { ref, onBeforeMount, computed } from "vue";
import { useRoute } from "vue-router";
import { errorMessages } from "vue/compiler-sfc";

const route = useRoute();

const formError = ref("");

const book = ref([]);
const students = ref([]);
onBeforeMount(async () => {
    book.value = (await db.select(`SELECT * FROM book WHERE PK_id=$1;`, [route.params.id]))[0];
    students.value = await db.select("SELECT * FROM student;");
});

const selectedStudentId = ref(undefined);
const searchTerm = ref("");
const filteredStudents = computed(() => {
    return students.value.filter(student => 
        student.firstname.toLowerCase().includes(searchTerm.value.toLowerCase())
    );
})

async function makeLoan() {
    if (selectedStudentId) {
        let studentLoans = await db.select("SELECT PK_id FROM loan WHERE FK_student = $1;", [selectedStudentId.value]);
        if (studentLoans.length === 0) {
            await db.execute("INSERT INTO loan (FK_book, FK_student) VALUES ($1, $2);", [route.params.id, selectedStudentId.value]).then((result) => {
                alert(`Parfait ! Le livre ${book.value.title} a été emprunté`)
            });
        } else {
            formError.value = "Vous avez déjà emprunté un livre";
        }
    } else {
        formError.value = "Sélectionnez votre prénom";
    }
}
</script>

<template>
    <nav>
        <RouterLink to="/">
            <navbar>
                <!-- Camera button -->
                <div class="navbar-button">
                    <img src="/images/back_icon.svg" alt="Icône de caméra">
                </div>
            </navbar>
        </RouterLink>
    </nav>
    <section>
        <div class="layout">
            <h1>{{ book.title }}</h1>
            <p v-if="formError" class="form-error-message">{{ formError }}</p>
            <input type="text" v-model="searchTerm" placeholder="Rechercher..." class="search-input" />
            <div class="student-list">
                <li v-for="student in filteredStudents" :key="student.PK_id" :class="{ selected: selectedStudentId === student.PK_id }" @click="selectedStudentId = student.PK_id">
                    {{ student.firstname }} - {{ student.name }}
                </li>
            </div>
            <button @click="makeLoan()">Emprunter</button>
        </div>
    </section>
</template>

<style scoped>
a {
    width: 100%;
}
.layout {
    backdrop-filter: blur(15px);
    background-color: rgba(0, 0, 0, 0.3);
    padding: 50px;
    border-radius: 30px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    height: 90%;
}
h1 {
    font-size: 5em;
    background-color: rgb(var(--vbiblio-blue));
    border-radius: 15px;
    padding: 20px;
    color: white;
    text-align: center;
}
section {
    padding: 60px;
    display: flex;
    justify-content: space-around;
    align-items: center;
    flex-direction: column;
    height: 100%;
}
.student-list {
    overflow: auto;
}
.search-input{
    height: 40px;
    margin: 20px 0;
    color: black;
    font-size: 2em;
}
button {
    appearance: none;
    padding: 15px;
    font-size: 1.5em;
    background-color: rgb(var(--vbiblio-blue));
    border-radius: 15px;
    border: none;
    cursor: pointer;
}
li {
    margin: 10px 0;
    border-bottom: 3px solid white;
    font-size: 2em;
}
.selected {
    background-color: rgb(var(--vbiblio-blue));
}
.form-error-message {
    color: #c01919;
}
</style>