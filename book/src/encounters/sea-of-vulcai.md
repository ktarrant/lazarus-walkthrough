<!-- area-id: sea-of-vulcai -->
### Sea of Vulcai

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | Underwater | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=charcadet">Charcadet</a> | 10% | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="charcadet" /> |
| <a href="../pokemon-lookup.html?q=fuecoco">Fuecoco</a> | 5% | — | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="fuecoco" /> |
| <a href="../pokemon-lookup.html?q=hisuian-voltorb">Hisuian Voltorb</a> | 10% | — | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="hisuian-voltorb" /> |
| <a href="../pokemon-lookup.html?q=kecleon">Kecleon</a> | 10% | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="kecleon" /> |
| <a href="../pokemon-lookup.html?q=kilowattrel">Kilowattrel</a> | 5% | 5% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="kilowattrel" /> |
| <a href="../pokemon-lookup.html?q=quaxly">Quaxly</a> | 5% | — | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="quaxly" /> |
| <a href="../pokemon-lookup.html?q=scyther">Scyther</a> | 20% | 20% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="scyther" /> |
| <a href="../pokemon-lookup.html?q=sizzlipede">Sizzlipede</a> | 10% | — | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="sizzlipede" /> |
| <a href="../pokemon-lookup.html?q=sprigatito">Sprigatito</a> | 5% | — | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="sprigatito" /> |
| <a href="../pokemon-lookup.html?q=tinkatuff">Tinkatuff</a> | 20% | 20% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="tinkatuff" /> |
| <a href="../pokemon-lookup.html?q=litten">Litten</a> | — | 5% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="litten" /> |
| <a href="../pokemon-lookup.html?q=noctowl">Noctowl</a> | — | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="noctowl" /> |
| <a href="../pokemon-lookup.html?q=popplio">Popplio</a> | — | 5% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="popplio" /> |
| <a href="../pokemon-lookup.html?q=rowlet">Rowlet</a> | — | 5% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="rowlet" /> |
| <a href="../pokemon-lookup.html?q=voltorb">Voltorb</a> | — | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="voltorb" /> |
| <a href="../pokemon-lookup.html?q=octillery">Octillery</a> | — | — | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="octillery" /> |
| <a href="../pokemon-lookup.html?q=remoraid">Remoraid</a> | — | — | 5% | — | — | — | 20% | <input type="checkbox" class="caught-check" data-species="remoraid" /> |
| <a href="../pokemon-lookup.html?q=tentacool">Tentacool</a> | — | — | 60% | 30% | — | — | — | <input type="checkbox" class="caught-check" data-species="tentacool" /> |
| <a href="../pokemon-lookup.html?q=wailmer">Wailmer</a> | — | — | 30% | — | — | 40% | 20% | <input type="checkbox" class="caught-check" data-species="wailmer" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | — | <input type="checkbox" class="caught-check" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=clamperl">Clamperl</a> | — | — | — | — | 20% | 40% | — | <input type="checkbox" class="caught-check" data-species="clamperl" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | — | — | 20% | — | 20% | <input type="checkbox" class="caught-check" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=psyduck">Psyduck</a> | — | — | — | — | 60% | — | — | <input type="checkbox" class="caught-check" data-species="psyduck" /> |
| <a href="../pokemon-lookup.html?q=chinchou">Chinchou</a> | — | — | — | — | — | 15% | — | <input type="checkbox" class="caught-check" data-species="chinchou" /> |
| <a href="../pokemon-lookup.html?q=lanturn">Lanturn</a> | — | — | — | — | — | 5% | — | <input type="checkbox" class="caught-check" data-species="lanturn" /> |
| <a href="../pokemon-lookup.html?q=skrelp">Skrelp</a> | — | — | — | — | — | — | 10% | <input type="checkbox" class="caught-check" data-species="skrelp" /> |

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

