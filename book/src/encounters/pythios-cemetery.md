<!-- area-id: pythios-cemetery -->
### Pythios Cemetery

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=bellsprout">Bellsprout</a> | 20% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="bellsprout" /> |
| <a href="../pokemon-lookup.html?q=cyndaquil">Cyndaquil</a> | 2% | 2% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="cyndaquil" /> |
| <a href="../pokemon-lookup.html?q=goomy">Goomy</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="goomy" /> |
| <a href="../pokemon-lookup.html?q=natu">Natu</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="natu" /> |
| <a href="../pokemon-lookup.html?q=paras">Paras</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="paras" /> |
| <a href="../pokemon-lookup.html?q=scraggy">Scraggy</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="scraggy" /> |
| <a href="../pokemon-lookup.html?q=shuppet">Shuppet</a> | 8% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="shuppet" /> |
| <a href="../pokemon-lookup.html?q=sizzlipede">Sizzlipede</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="sizzlipede" /> |
| <a href="../pokemon-lookup.html?q=trapinch">Trapinch</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="trapinch" /> |
| <a href="../pokemon-lookup.html?q=wattrel">Wattrel</a> | 20% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="wattrel" /> |
| <a href="../pokemon-lookup.html?q=duskull">Duskull</a> | — | 8% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="duskull" /> |
| <a href="../pokemon-lookup.html?q=ekans">Ekans</a> | — | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="ekans" /> |
| <a href="../pokemon-lookup.html?q=litwick">Litwick</a> | — | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="litwick" /> |
| <a href="../pokemon-lookup.html?q=snom">Snom</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="snom" /> |
| <a href="../pokemon-lookup.html?q=stunky">Stunky</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="stunky" /> |
| <a href="../pokemon-lookup.html?q=clauncher">Clauncher</a> | — | — | 10% | — | — | — | <input type="checkbox" class="caught-check" data-species="clauncher" /> |
| <a href="../pokemon-lookup.html?q=remoraid">Remoraid</a> | — | — | 30% | — | — | — | <input type="checkbox" class="caught-check" data-species="remoraid" /> |
| <a href="../pokemon-lookup.html?q=wimpod">Wimpod</a> | — | — | 60% | — | — | — | <input type="checkbox" class="caught-check" data-species="wimpod" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="caught-check" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=clamperl">Clamperl</a> | — | — | — | — | 20% | — | <input type="checkbox" class="caught-check" data-species="clamperl" /> |
| <a href="../pokemon-lookup.html?q=dratini">Dratini</a> | — | — | — | — | 20% | 15% | <input type="checkbox" class="caught-check" data-species="dratini" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | — | — | 60% | — | <input type="checkbox" class="caught-check" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=dragonair">Dragonair</a> | — | — | — | — | — | 5% | <input type="checkbox" class="caught-check" data-species="dragonair" /> |
| <a href="../pokemon-lookup.html?q=poliwhirl">Poliwhirl</a> | — | — | — | — | — | 40% | <input type="checkbox" class="caught-check" data-species="poliwhirl" /> |

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

