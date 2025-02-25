import "./App.css";
import { GitRepositories } from "./routes/GitRepositories";
import { PullRequests } from "./routes/PullRequests";
import { Root } from "./routes/__root";
import { Store } from "./store/store";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { RouterProvider, createRouter } from "@tanstack/react-router";
import { StoreProvider } from "easy-peasy";

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
      <StoreProvider store={Store}>
        <RouterProvider router={router} />
      </StoreProvider>
    </QueryClientProvider>
  );
}

export default App;
