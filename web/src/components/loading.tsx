export const LoadingBar = () => (
  <svg class='h-1 w-full' viewBox='0 0 10 1' preserveAspectRatio='none'>
    <title>Loading...</title>
    <rect width='10' height='1' color='#9CA3AF' />
    <rect width='1' height='1' color='#111827'>
      <animateMotion repeatCount='indefinite' dur='2s' path='M-1,0 L10,0' />
    </rect>
  </svg>
);
