import { Outlet } from 'react-router';
import { Footer } from '~/components/Footer/Footer';
import { Header } from '~/components/Header/Header';

export default function MainLayout() {
  return (
    <div className="flex flex-col items-center min-h-screen">
      <Header />
      <main className="flex flex-col justify-center flex-1">
        <Outlet />
      </main>
      <Footer />
    </div>
  );
}
