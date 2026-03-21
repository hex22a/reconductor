import { Outlet, useNavigate } from 'react-router';
import { Footer } from '~/components/Footer/Footer';
import { Header } from '~/components/Header/Header';
import { useAuth } from '~/providers/AuthProvider';

export default function DashboardLayout() {
  const navigate = useNavigate();
  const { user } = useAuth();
  if (!user) {
    navigate('/signin');
  }
  return (
    <div className="flex flex-col items-center min-h-screen">
      <Header />
      <main className="flex flex-col flex-1">
        <Outlet />
      </main>
      <Footer />
    </div>
  );
}
