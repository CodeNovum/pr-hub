import "./App.css";
import { GitRepositories } from "./routes/GitRepositories";
import { PullRequests } from "./routes/PullRequests";
import { Root } from "./routes/__root";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { RouterProvider, createRouter } from "@tanstack/react-router";
import { Slide, ToastContainer } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";

const routeTree = Root.addChildren([GitRepositories, PullRequests]);
const router = createRouter({ routeTree });
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
    },
  },
});

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <ReactQueryDevtools initialIsOpen={false} buttonPosition="bottom-right" />
      <RouterProvider router={router} />
      <ToastContainer
        closeButton={false}
        closeOnClick
        autoClose={3000}
        transition={Slide}
        hideProgressBar
      />
    </QueryClientProvider>
  );
}

export default App;
