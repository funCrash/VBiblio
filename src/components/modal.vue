<script setup>
import { ref, onMounted, watch, nextTick } from "vue";
import { vOnClickOutside } from "@vueuse/components";
import  QrScanner from 'qr-scanner';
import { useRouter } from "vue-router";

const router = useRouter()
// To open and close the modal
const props = defineProps({
    isModalOpen: Boolean,
});
const emit = defineEmits(["closeModal"]);
function closeModal() {
    emit('closeModal');
    qrScanner.stop()
}

// To use the QRCode scanner
let qrScanner = undefined;
const qrcodeArea = ref(null);
onMounted(() => {
    watch(() => props.isModalOpen, async (isModalOpen) => {
        if (isModalOpen) {
            await nextTick();
            try {
                qrScanner = new QrScanner(
                    qrcodeArea.value,
                    result => {
                        console.log('decoded qr code:', result);
                        router.push({ path: "book/", params: { id:result.data } })
                    },
                    { highlightScanRegion: true },
                );
                qrScanner.start();
            } catch (error) {
                console.log("Error starting QR scanner. Probably permission denied. See details :", error)
            }
        }
    })
});
</script>

<template>
    <Teleport to="body" v-if="props.isModalOpen">
        <div class="modal-bg">
            <div class="modal" v-on-click-outside="closeModal">
                <video ref="qrcodeArea"></video>
            </div>
        </div>
    </Teleport>
</template>

<style scoped>
.modal-bg {
    z-index: 3;
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;

    /* Darken the screen */
    background-color: rgba(0, 0, 0, 0.5);

    /* Center the modal itself */
    display: flex;
    justify-content: center;
    align-items: center;
}
.modal {
    background-color: rgb(var(--vbiblio-blue));
    width: 70%;
    height: 70%;
    border-radius: 10px;
    box-shadow: 0 10px 5px 2px rgba(0, 0, 0, 0.1);
    display: flex;
    justify-content: center;
    align-items: center;
}
video {
    max-width: 95%;
    max-height: 95%;
}
</style>