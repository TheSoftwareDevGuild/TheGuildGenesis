import { Award, Badge } from "lucide-react";
import { useGetAttestations } from "@/hooks/attestations/use-get-attestations";
import { useMemo } from "react";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { useGetProfiles } from "@/hooks/profiles/use-get-profiles";
import CopyAddressToClipboard from "@/components/CopyAddressToClipboard";

export function ProfileAttestations({ address }: { address: string }) {
  const attestationsQuery = useGetAttestations();
  const profilesQuery = useGetProfiles();

  const attestations = useMemo(() => {
    const list = attestationsQuery.data ?? [];
    const filtered = list.filter(
      (a) => a.recipient.toLowerCase() === (address || "").toLowerCase()
    );
    return filtered.map((a, i) => ({
      id: String(i),
      badgeName: a.badgeName,
      justification: a.attestationJustification,
      issuer: a.issuer,
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
      { id: string; issuer: string; justification: string }[]
    >();
    for (const a of attestations) {
      const arr = map.get(a.badgeName) ?? [];
      arr.push({ id: a.id, issuer: a.issuer, justification: a.justification });
      map.set(a.badgeName, arr);
    }
    return Array.from(map.entries()).map(([badgeName, items]) => ({
      badgeName,
      items,
    }));
  }, [attestations]);
  return (
    <section className="mt-10">
      <h2 className="text-lg font-medium mb-4 flex items-center gap-2">
        <Award className="h-5 w-5" /> Attestations ({attestations.length})
      </h2>
      {attestations.length === 0 ? (
        <p className="text-sm text-gray-600">No attestations yet.</p>
      ) : (
        <Accordion type="multiple" className="w-full space-y-2">
          {grouped.map(({ badgeName, items }, idx) => (
            <AccordionItem
              key={badgeName || idx}
              value={badgeName || String(idx)}
            >
              <AccordionTrigger>
                <div className="flex items-center gap-2">
                  <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
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
                          <div className="text-xs text-gray-500 mt-1 flex items-center gap-1">
                            <span>Issued by{" "}</span>
                            <a
                              className="text-indigo-600 hover:underline"
                              href={`/profiles/${it.issuer}`}
                            >
                              {profileNameByAddress.get(
                                it.issuer.toLowerCase()
                              ) || `${it.issuer.slice(0, 6)}...${it.issuer.slice(-4)}`}
                            </a>
                            <CopyAddressToClipboard 
                              address={it.issuer}
                              iconSize="sm"
                              showFeedback={false}
                            />
                          </div>
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

export default ProfileAttestations;
