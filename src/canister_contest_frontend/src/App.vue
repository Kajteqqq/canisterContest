<script setup>
import { ref } from 'vue';
import { canister_contest_backend } from 'declarations/canister_contest_backend/index';

// Definiujemy zmienne i ich referencje
const wyswietlacz = ref("0");  // Używamy ref dla reaktywności w Vue
const liczba1 = ref("");   // Używamy ref dla reaktywności
const liczba2 = ref("");   // Używamy ref dla reaktywności
const op = ref("");  // Operator
const counter = ref(0);   // Używamy ref dla reaktywności

function dodajCyfre(cyfra) {
    if (counter.value === 0) {
        liczba1.value = liczba1.value === "0" ? cyfra : liczba1.value + cyfra;
        wyswietlacz.value = liczba1.value + (op.value || "");
    } else if (counter.value === 1) {
        liczba2.value = liczba2.value === "" ? cyfra : liczba2.value + cyfra;
        wyswietlacz.value = liczba1.value + " " + op.value + " " + liczba2.value;
    }
}

function ustawOperator(operator) {
    if (counter.value === 0 && liczba1.value !== "") {
        op.value = operator;
        wyswietlacz.value = liczba1.value + " " + op.value;  // Aktualizujemy wyświetlacz z operatorem
        counter.value = 1;  // Ustawiamy licznik na 1, aby wskazać, że operator jest ustawiony
    } else if (counter.value === 1 && liczba2.value !== "") {
        oblicz();  // Obliczamy, jeśli operator już jest ustawiony i mamy dwie liczby
        op.value = operator;
        wyswietlacz.value = liczba1.value + " " + op.value;
    } else {
        console.log("Najpierw ustaw pierwszą liczbę.");
    }
}

function zresetuj() {
    liczba1.value = "";
    liczba2.value = "";
    op.value = "";
    counter.value = 0;
    wyswietlacz.value = "0";
}

// Używamy BigInt dla dużych liczb, aby pasowały do nat32 w IC
async function dodaj(liczba1, liczba2) {
    const num1 = Math.round(parseFloat(liczba1));
    const num2 = Math.round(parseFloat(liczba2));
    return await canister_contest_backend.dodaj_liczby(num1, num2);
}

async function odejmij(liczba1, liczba2) {
    const num1 = Math.round(parseFloat(liczba1));
    const num2 = Math.round(parseFloat(liczba2));
    return await canister_contest_backend.odejmij_liczby(num1, num2);
}

async function pomnoz(liczba1, liczba2) {
    const num1 = Math.round(parseFloat(liczba1));
    const num2 = Math.round(parseFloat(liczba2));
    return await canister_contest_backend.pomnoz_liczby(num1, num2);
}

async function podziel(liczba1, liczba2) {
    const num1 = Math.round(parseFloat(liczba1));
    const num2 = Math.round(parseFloat(liczba2));
    return await canister_contest_backend.podziel_liczby(num1, num2);
}

async function oblicz() {
    if (liczba1.value && liczba2.value && op.value) {
        let wynik;
        switch (op.value) {
            case "+":
                wynik = await dodaj(liczba1.value, liczba2.value);
                break;
            case "-":
                wynik = await odejmij(liczba1.value, liczba2.value);
                break;
            case "*":
                wynik = await pomnoz(liczba1.value, liczba2.value);
                break;
            case "/":
                wynik = await podziel(liczba1.value, liczba2.value);
                break;
            default:
                console.log("Nieznany operator.");
                return;
        }
        wyswietlacz.value = wynik.toString();
        liczba1.value = wynik.toString();  // Ustawiamy wynik jako pierwszą liczbę dla dalszych obliczeń
        liczba2.value = "";   // Resetujemy drugą liczbę
        op.value = "";        // Resetujemy operator
        counter.value = 0;    // Resetujemy licznik
    }
}
</script>

<template>
<center>
  <h1>Prosty kalkulator</h1>
</center>
<div id="app" class="calculator">
    <div class="display">
        {{ wyswietlacz }}
    </div>
    <div class="buttons">
        <button @click="dodajCyfre('7')" class="button">7</button>
        <button @click="dodajCyfre('8')" class="button">8</button>
        <button @click="dodajCyfre('9')" class="button">9</button>
        <button @click="ustawOperator('/')" class="button operator">/</button>
        <button @click="dodajCyfre('4')" class="button">4</button>
        <button @click="dodajCyfre('5')" class="button">5</button>
        <button @click="dodajCyfre('6')" class="button">6</button>
        <button @click="ustawOperator('*')" class="button operator">*</button>
        <button @click="dodajCyfre('1')" class="button">1</button>
        <button @click="dodajCyfre('2')" class="button">2</button>
        <button @click="dodajCyfre('3')" class="button">3</button>
        <button @click="ustawOperator('-')" class="button operator">-</button>
        <button @click="dodajCyfre('0')" class="button">0</button>
        <button @click="oblicz()" class="button equal">=</button> 
        <button @click="zresetuj()" class="button clear">C</button>
        <button @click="ustawOperator('+')" class="button operator">+</button>
    </div>
</div>
</template>
