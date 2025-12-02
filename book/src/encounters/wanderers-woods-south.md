<!-- area-id: wanderers-woods-south -->
### Wanderer's Woods (South)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=aipom">Aipom</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="aipom" /> |
| <a href="../pokemon-lookup.html?q=goomy">Goomy</a> | 8% | 8% | <input type="checkbox" class="caught-check" data-species="goomy" /> |
| <a href="../pokemon-lookup.html?q=heracross">Heracross</a> | 5% | — | <input type="checkbox" class="caught-check" data-species="heracross" /> |
| <a href="../pokemon-lookup.html?q=lileep">Lileep</a> | 5% | — | <input type="checkbox" class="caught-check" data-species="lileep" /> |
| <a href="../pokemon-lookup.html?q=munna">Munna</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="munna" /> |
| <a href="../pokemon-lookup.html?q=natu">Natu</a> | 20% | 20% | <input type="checkbox" class="caught-check" data-species="natu" /> |
| <a href="../pokemon-lookup.html?q=paras">Paras</a> | 20% | 20% | <input type="checkbox" class="caught-check" data-species="paras" /> |
| <a href="../pokemon-lookup.html?q=ralts">Ralts</a> | 10% | 10% | <input type="checkbox" class="caught-check" data-species="ralts" /> |
| <a href="../pokemon-lookup.html?q=scyther">Scyther</a> | 2% | 2% | <input type="checkbox" class="caught-check" data-species="scyther" /> |
| <a href="../pokemon-lookup.html?q=wooper">Wooper</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="wooper" /> |
| <a href="../pokemon-lookup.html?q=anorith">Anorith</a> | — | 5% | <input type="checkbox" class="caught-check" data-species="anorith" /> |
| <a href="../pokemon-lookup.html?q=duskull">Duskull</a> | — | 10% | <input type="checkbox" class="caught-check" data-species="duskull" /> |
| <a href="../pokemon-lookup.html?q=impidimp">Impidimp</a> | — | 10% | <input type="checkbox" class="caught-check" data-species="impidimp" /> |
| <a href="../pokemon-lookup.html?q=oricorio-sensu">Oricorio Sensu</a> | — | 5% | <input type="checkbox" class="caught-check" data-species="oricorio-sensu" /> |
| <a href="../pokemon-lookup.html?q=paldean-wooper">Paldean Wooper</a> | — | 10% | <input type="checkbox" class="caught-check" data-species="paldean-wooper" /> |

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

