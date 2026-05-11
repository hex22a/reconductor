<script lang="ts">
  interface Props {
    className?: string;
    height?: number;
    width?: number;
  }
  let { height = 24, width = 24, className }: Props = $props();
</script>

<svg xmlns="http://www.w3.org/2000/svg" fill="none" class={className} {width} {height}>
  <path
    d="M16.288 14.962a.939.939 0 1 1-1.328 1.328L10 11.328l-4.962 4.96A.94.94 0 0 1 3.71 14.96L8.672 10l-4.96-4.962A.94.94 0 1 1 5.04 3.71L10 8.672l4.962-4.963a.94.94 0 1 1 1.328 1.328L11.328 10l4.96 4.962Z"
  />
</svg>
