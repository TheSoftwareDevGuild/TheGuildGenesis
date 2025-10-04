import { Send, Badge } from "lucide-react";
import { useGetAttestations } from "@/hooks/attestations/use-get-attestations";
import { useMemo } from "react";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { useGetProfiles } from "@/hooks/profiles/use-get-profiles";

export function ProfileIssuedAttestations({ address }: { address: string }) {
  const attestationsQuery = useGetAttestations();
  const profilesQuery = useGetProfiles();

  const issuedAttestations = useMemo(() => {
    const list = attestationsQuery.data ?? [];
    const filtered = list.filter(
      (a) => a.issuer.toLowerCase() === (address || "").toLowerCase()
    );
    return filtered.map((a, i) => ({
      id: String(i),
      badgeName: a.badgeName,
      justification: a.attestationJustification,
      recipient: a.recipient,
    }));
  }, [attestationsQuery.data, address]);

  const profileNameByAddress = useMemo(() => {
    const map = new Map<string, string>();
    const list = profilesQuery.data ?? [];
    for (const p of list) {
      if (p.address) map.set(p.address.toLowerCase(), p.name || "");
    }
    return map;
  }, [profilesQuery.data]);

  const grouped = useMemo(() => {
    const map = new Map<
      string,
      { id: string; recipient: string; justification: string }[]
    >();
    for (const a of issuedAttestations) {
      const arr = map.get(a.badgeName) ?? [];
      arr.push({ id: a.id, recipient: a.recipient, justification: a.justification });
      map.set(a.badgeName, arr);
    }
    return Array.from(map.entries()).map(([badgeName, items]) => ({
      badgeName,
      items,
    }));
  }, [issuedAttestations]);

  return (
    <section className="mt-10">
      <h2 className="text-lg font-medium mb-4 flex items-center gap-2">
        <Send className="h-5 w-5" /> Issued Attestations ({issuedAttestations.length})
      </h2>
      {issuedAttestations.length === 0 ? (
        <p className="text-sm text-gray-600">No attestations issued yet.</p>
      ) : (
        <Accordion type="multiple" className="w-full space-y-2">
          {grouped.map(({ badgeName, items }, idx) => (
            <AccordionItem
              key={badgeName || idx}
              value={badgeName || String(idx)}
            >
              <AccordionTrigger>
                <div className="flex items-center gap-2">
                  <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
                    <Badge className="h-3 w-3 mr-1" />{" "}
                    {badgeName || "(unnamed)"}
                  </span>
                  <span className="text-sm text-gray-600">
                    ({items.length})
                  </span>
                </div>
              </AccordionTrigger>
              <AccordionContent>
                <ul className="space-y-3">
                  {items.map((it) => (
                    <li
                      key={it.id}
                      className="border rounded-lg p-4 flex items-start justify-between"
                    >
                      <div className="flex items-start gap-3">
                        <div>
                          <p className="text-sm text-gray-800">
                            {it.justification}
                          </p>
                          <p className="text-xs text-gray-500 mt-1">
                            Issued to{" "}
                            <a
                              className="text-indigo-600 hover:underline"
                              href={`/profiles/${it.recipient}`}
                            >
                              {profileNameByAddress.get(
                                it.recipient.toLowerCase()
                              ) || `${it.recipient.slice(0, 6)}...${it.recipient.slice(-4)}`}
                            </a>
                          </p>
                        </div>
                      </div>
                    </li>
                  ))}
                </ul>
              </AccordionContent>
            </AccordionItem>
          ))}
        </Accordion>
      )}
    </section>
  );
}

export default ProfileIssuedAttestations;