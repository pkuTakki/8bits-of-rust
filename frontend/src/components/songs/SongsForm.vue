<template>
  <my-input v-model="songName" placeholder="请输入歌曲名称..." />
  <br>
  <SongsList ref="songsListRef" />
</template>

<script>
import SongsList from './SongsList.vue';
import MyInput from '@/components/utils/MyInput.vue';
export default {
  name: "SongsForm",
  data() {
    return {
      songName: '',
    };
  },
  mounted() {
      this.$nextTick(() => {
          console.log('songsForm已初始化:', this.$refs.songsListRef.songs)
      })
  },
  components: {
    SongsList,
    MyInput,
  },
  methods: {
    addItem() {
      if (this.songName.trim()) {
        this.$refs.songsListRef.addItem(this.songName);
        console.log("SongsForm addItem:", this.songName);
      }
    },
  },
};
</script>

<style scoped>
</style>