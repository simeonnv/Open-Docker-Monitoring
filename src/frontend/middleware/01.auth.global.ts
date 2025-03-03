export default defineNuxtRouteMiddleware(async (to, from) => {
  const { $pinia } = useNuxtApp(); // Access the Pinia instance
  const authStore = useAuthStore($pinia); // Pass the Pinia instance explicitly

  const main_account_exists = await authStore.MainAccountExists();
  const auth_validated = await authStore.ValidateAuth()

  if ((to.path === '/login' || to.path === '/signup') && auth_validated)
    return navigateTo('/');
    

  if (to.path === '/login')
    if (main_account_exists)
      return
    else
      return navigateTo('/signup');

  if (to.path === '/signup')
    if (main_account_exists)
      return navigateTo('/login');
    else
      return


  if (auth_validated) {
    return; // Proceed if authenticated
  } else {
    if (main_account_exists)
      return navigateTo('/login');
    else
      return navigateTo('/signup');
  }
});