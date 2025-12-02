<!-- area-id: areios-hideout -->
### Areios Hideout

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=ariados">Ariados</a> | 20% | — | <input type="checkbox" class="caught-check" data-species="ariados" /> |
| <a href="../pokemon-lookup.html?q=boltund">Boltund</a> | 10% | 10% | <input type="checkbox" class="caught-check" data-species="boltund" /> |
| <a href="../pokemon-lookup.html?q=houndoom">Houndoom</a> | 20% | 20% | <input type="checkbox" class="caught-check" data-species="houndoom" /> |
| <a href="../pokemon-lookup.html?q=lurantis">Lurantis</a> | 8% | — | <input type="checkbox" class="caught-check" data-species="lurantis" /> |
| <a href="../pokemon-lookup.html?q=muk">Muk</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="muk" /> |
| <a href="../pokemon-lookup.html?q=obstagoon">Obstagoon</a> | 10% | 10% | <input type="checkbox" class="caught-check" data-species="obstagoon" /> |
| <a href="../pokemon-lookup.html?q=primeape">Primeape</a> | 10% | 10% | <input type="checkbox" class="caught-check" data-species="primeape" /> |
| <a href="../pokemon-lookup.html?q=salandit">Salandit</a> | 10% | 10% | <input type="checkbox" class="caught-check" data-species="salandit" /> |
| <a href="../pokemon-lookup.html?q=tyrunt">Tyrunt</a> | 2% | 2% | <input type="checkbox" class="caught-check" data-species="tyrunt" /> |
| <a href="../pokemon-lookup.html?q=alolan-muk">Alolan Muk</a> | — | 10% | <input type="checkbox" class="caught-check" data-species="alolan-muk" /> |
| <a href="../pokemon-lookup.html?q=drapion">Drapion</a> | — | 20% | <input type="checkbox" class="caught-check" data-species="drapion" /> |
| <a href="../pokemon-lookup.html?q=scrafty">Scrafty</a> | — | 8% | <input type="checkbox" class="caught-check" data-species="scrafty" /> |

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

