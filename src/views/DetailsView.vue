<script setup>
import navbar from "../components/navbar.vue";
import loadDatabase from "../shared/SQL_API";
import { ref, onBeforeMount, computed } from "vue";
import { useRoute } from "vue-router";
import { errorMessages } from "vue/compiler-sfc";
import { useRouter } from "vue-router";

const router = useRouter()
const route = useRoute();
let db
const formError = ref("");

/* Initial component data */
const book = ref([]);
const studentList = ref([]);
const loan = ref([]);
const isBookLoaned = ref(false);
onBeforeMount(async () => {
    db = await loadDatabase();
    book.value = (await db.select("SELECT * FROM book WHERE PK_id=$1;", [route.params.id]))[0];
    loan.value = (await db.select("SELECT name, firstname FROM loan INNER JOIN student ON loan.FK_student = student.PK_id WHERE (loan.FK_book = $1 AND loan.returned = 0);", [route.params.id]))[0];
    if (loan.value) { // If the book is already loaned
        isBookLoaned.value = true;
    } else {
        studentList.value = await db.select("SELECT * FROM student;");
    }
});

/* Student selection field */
const selectedStudentId = ref(undefined);
const searchTerm = ref("");
const filteredStudents = computed(() => {
    return studentList.value.filter(student => 
        student.firstname.toLowerCase().includes(searchTerm.value.toLowerCase())
    );
})

/* Fonctions to make a loan */
const formatDate = function(dateObject) {
    const year = String(dateObject.getFullYear());
    const month = String(dateObject.getMonth() + 1).padStart(2, "0");
    const day = String(dateObject.getDate()).padStart(2, '0');
    const hours = String(dateObject.getHours()).padStart(2, '0');
    const minutes = String(dateObject.getMinutes()).padStart(2, '0');
    const seconds = String(dateObject.getSeconds()).padStart(2, '0');

    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
}

async function makeLoan() {
    if (selectedStudentId) {
        let studentLoans = await db.select("SELECT PK_id FROM loan WHERE (FK_student = $1 AND returned = 0);", [selectedStudentId.value]);
        if (studentLoans.length === 0) {
            await db.execute("INSERT INTO loan (FK_book, FK_student, loan_date) VALUES ($1, $2, $3);", [route.params.id, selectedStudentId.value, formatDate(new Date())]).then((result) => {
                alert(`Parfait ! Le livre ${book.value.title} a été emprunté`)
            });
        } else {
            formError.value = "Vous avez déjà emprunté un livre";
        }
    } else {
        formError.value = "Sélectionnez votre prénom";
    }
}
async function deleteLoan() {
    await db.execute("UPDATE loan SET returned=1, returned_date=$1 WHERE FK_book=$2;", [formatDate(new Date()), route.params.id]).then((result) => {
        alert("Le livre a bien été rendu");
        router.push({ path: "/" })
    });
}
</script>

<template>
    <nav>
        <RouterLink to="/">
            <navbar>
                <div class="navbar-button">
                    <img src="/images/back_icon.svg" alt="Icône de caméra">
                </div>
            </navbar>
        </RouterLink>
    </nav>
    <!-- If the book is already loaned -->
    <section v-if="isBookLoaned">
        <div class="layout">
            <h1>{{ book.title }}</h1>
            <p>Ce livre a été emprunté par {{ loan.firstname }} {{ loan.name }}</p>
            <button @click="deleteLoan()">Rendre</button>
        </div>
    </section>

    <!-- If the book isn't loaned -->
    <section v-else>
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
