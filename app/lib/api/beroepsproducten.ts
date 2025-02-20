import { Architectuurlaag } from "../../types/Architectuurlaag";
import { Activiteit } from "../../types/Activiteit";

export type CreateBeroepsproductDTO = {
  title: string;
  layer: Architectuurlaag | "";
  activity: Activiteit | "";
  guild: string;
  sublament: string;
  level: number;
};

export async function createBeroepsproduct(data: CreateBeroepsproductDTO) {
  const response = await fetch("/api/v1/beroepsproducten", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(data),
  });

  if (!response.ok) {
    throw new Error("Failed to create beroepsproduct");
  }

  return response.json();
}

export async function updateBeroepsproduct(
  id: string,
  data: CreateBeroepsproductDTO,
) {
  const response = await fetch(`/api/v1/beroepsproducten/${id}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(data),
  });

  if (!response.ok) {
    throw new Error("Failed to create beroepsproduct");
  }

  return response.json();
}

export async function fetchBeroepsproducten() {
  const response = await fetch("/api/v1/beroepsproducten");

  if (!response.ok) {
    throw new Error("Failed to fetch beroepsproducten");
  }

  return response.json();
}
