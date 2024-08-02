<script setup>
import { ref } from 'vue';
import { canister_contest_backend } from 'declarations/canister_contest_backend/index';

// Common Reactive State
const wyswietlacz = ref("0");
const liczba1 = ref("");
const liczba2 = ref("");
const op = ref("");
const counter = ref(0);

// Delta Calculator Specific State
const wyswietlaczDelta = ref("0");
const deltaA = ref("");
const deltaB = ref("");
const deltaC = ref("");

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
        wyswietlacz.value = liczba1.value + " " + op.value;
        counter.value = 1;
    } else if (counter.value === 1 && liczba2.value !== "") {
        oblicz();
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
    if (num2 === 0) {
        console.error("Nie można dzielić przez zero");
        return "Error";
    }
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
        liczba1.value = wynik.toString();
        liczba2.value = "";
        op.value = "";
        counter.value = 0;
    }
}
//delta
function dodajCyfreDelta(cyfra) {
    if (deltaA.value === "") {
        deltaA.value = cyfra;
    } else if (deltaB.value === "") {
        deltaB.value = cyfra;
    } else if (deltaC.value === "") {
        deltaC.value = cyfra;
    }
}

async function obliczDelte() {
    if (deltaA.value && deltaB.value && deltaC.value) {
        const wynik = await canister_contest_backend.oblicz_delte(
            parseInt(deltaA.value, 10),
            parseInt(deltaB.value, 10),
            parseInt(deltaC.value, 10)
        );
        wyswietlaczDelta.value = wynik.toString();
    }
}
function czyscDelta() {
    deltaA.value = "";
    deltaB.value = "";
    deltaC.value = "";
    wyswietlaczDelta.value = "0";
}

//pierwiastki

const wyswietlaczPierwiastki = ref("0");
const liczbaPierwiastka = ref("");

async function obliczPierwiastekKwadratowy() {
    if (liczbaPierwiastka.value !== "") {
        try {
            const wynik = await canister_contest_backend.oblicz_pierwiastek_kwadratowy(parseFloat(liczbaPierwiastka.value));
            wynik.toFixed(2);
            wyswietlaczPierwiastki.value = Math.round(wynik.toString());
        } catch (error) {
            wyswietlaczPierwiastki.value = "Błąd";
            console.error("Błąd obliczania pierwiastka kwadratowego:", error);
        }
    }
}

async function obliczPierwiastekSzescienny() {
    if (liczbaPierwiastka.value !== "") {
        const wynik = await canister_contest_backend.oblicz_pierwiastek_szescienny(parseFloat(liczbaPierwiastka.value));
        wynik.toFixed(2);
        wyswietlaczPierwiastki.value = Math.round(wynik.toString());
    }
}

function czyscPierwiastek() {
    liczbaPierwiastka.value = "";
    wyswietlaczPierwiastki.value = "0";
}
</script>


<template>
    <body>
      <div class="calculator-container">
        <div class="calculator">
          <center id="help">
            <h1 class="titles">Prosty kalkulator</h1>
          </center>
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
  
        <div id="deltaCalculator" class="calculator">
          <center>
            <h1 class="titles">Kalkulator Delty</h1>
          </center>
          <div class="display">
            {{ wyswietlaczDelta }}
          </div>
          <div class="deltaNumbers">
            <input class="deltaNumber" type="text" v-model="deltaA" readonly>
            <input class="deltaNumber" type="text" v-model="deltaB" readonly>
            <input class="deltaNumber" type="text" v-model="deltaC" readonly>
          </div>
          <div class="deltaButtons">
            <button @click="dodajCyfreDelta('7')" class="button">7</button>
            <button @click="dodajCyfreDelta('8')" class="button">8</button>
            <button @click="dodajCyfreDelta('9')" class="button">9</button>
            <button @click="dodajCyfreDelta('4')" class="button">4</button>
            <button @click="dodajCyfreDelta('5')" class="button">5</button>
            <button @click="dodajCyfreDelta('6')" class="button">6</button>
            <button @click="dodajCyfreDelta('1')" class="button">1</button>
            <button @click="dodajCyfreDelta('2')" class="button">2</button>
            <button @click="dodajCyfreDelta('3')" class="button">3</button>
            <button @click="dodajCyfreDelta('0')" class="button">0</button>
            <button @click="obliczDelte()" class="button equal">=</button>
            <button @click="czyscDelta()" class="button">C</button>
          </div>
        </div>
  
        <div id="rootCalculator" class="calculator">
          <center>
            <h1 class="titles">Kalkulator pierwiastków</h1>
          </center>
          <div class="display">
            {{ wyswietlaczPierwiastki }}
          </div>
          <div class="rootInput">
            <input type="number" v-model="liczbaPierwiastka" placeholder="Wprowadź liczbę" class="input-number">
          </div>
          <div class="rootButtons">
            <button @click="obliczPierwiastekKwadratowy()" class="button equal">√x</button>
            <button @click="obliczPierwiastekSzescienny()" class="button equal">∛x</button>
            <button @click="czyscPierwiastek()" class="button clear">C</button>
          </div>
        </div>
      </div>
    </body>
  </template>
  