<!-- area-id: lastlight-road -->
### Lastlight Road

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=armarouge">Armarouge</a> | 2% | — | <input type="checkbox" class="caught-check" data-species="armarouge" /> |
| <a href="../pokemon-lookup.html?q=breloom">Breloom</a> | 5% | 5% | <input type="checkbox" class="caught-check" data-species="breloom" /> |
| <a href="../pokemon-lookup.html?q=cherrim">Cherrim</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="cherrim" /> |
| <a href="../pokemon-lookup.html?q=cyclizar">Cyclizar</a> | 5% | 5% | <input type="checkbox" class="caught-check" data-species="cyclizar" /> |
| <a href="../pokemon-lookup.html?q=helioptile">Helioptile</a> | 10% | 10% | <input type="checkbox" class="caught-check" data-species="helioptile" /> |
| <a href="../pokemon-lookup.html?q=lampent">Lampent</a> | 20% | 20% | <input type="checkbox" class="caught-check" data-species="lampent" /> |
| <a href="../pokemon-lookup.html?q=litleo">Litleo</a> | 20% | — | <input type="checkbox" class="caught-check" data-species="litleo" /> |
| <a href="../pokemon-lookup.html?q=mudbray">Mudbray</a> | 10% | 10% | <input type="checkbox" class="caught-check" data-species="mudbray" /> |
| <a href="../pokemon-lookup.html?q=ponyta">Ponyta</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="ponyta" /> |
| <a href="../pokemon-lookup.html?q=pyroar-f">Pyroar F</a> | 4% | 4% | <input type="checkbox" class="caught-check" data-species="pyroar-f" /> |
| <a href="../pokemon-lookup.html?q=pyroar-m">Pyroar M</a> | 4% | 4% | <input type="checkbox" class="caught-check" data-species="pyroar-m" /> |
| <a href="../pokemon-lookup.html?q=applin">Applin</a> | — | 10% | <input type="checkbox" class="caught-check" data-species="applin" /> |
| <a href="../pokemon-lookup.html?q=ceruledge">Ceruledge</a> | — | 2% | <input type="checkbox" class="caught-check" data-species="ceruledge" /> |
| <a href="../pokemon-lookup.html?q=flaaffy">Flaaffy</a> | — | 20% | <input type="checkbox" class="caught-check" data-species="flaaffy" /> |
| <a href="../pokemon-lookup.html?q=galarian-ponyta">Galarian Ponyta</a> | — | 10% | <input type="checkbox" class="caught-check" data-species="galarian-ponyta" /> |

<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>

