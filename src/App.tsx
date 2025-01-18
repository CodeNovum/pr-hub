import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import "./App.css";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { StoreProvider } from "easy-peasy";
import { Store } from "./store/store";
import { createRouter, RouterProvider } from "@tanstack/react-router";
import { Root } from "./routes/__root";
import { Organizations } from "./routes/Organizations";
import { PullRequests } from "./routes/PullRequests";

const routeTree = Root.addChildren([Organizations, PullRequests]);
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
